use crate::{
    codec::{EncoderApi, EncoderCfg},
    hw, ImageFormat, HW_STRIDE_ALIGN,
};
use hbb_common::{
    allow_err,
    anyhow::{anyhow, Context},
    bytes::Bytes,
    config::HwCodecConfig,
    log,
    message_proto::{EncodedVideoFrame, EncodedVideoFrames, Message, VideoFrame},
    ResultType,
};
use hwcodec::{
    decode::{DecodeContext, DecodeFrame, Decoder},
    encode::{EncodeContext, EncodeFrame, Encoder},
    ffmpeg::{CodecInfo, CodecInfos, DataFormat},
    AVPixelFormat,
    Quality::{self, *},
    RateControl::{self, *},
};

const CFG_KEY_ENCODER: &str = "bestHwEncoders";
const CFG_KEY_DECODER: &str = "bestHwDecoders";

const DEFAULT_PIXFMT: AVPixelFormat = AVPixelFormat::AV_PIX_FMT_YUV420P;
pub const DEFAULT_TIME_BASE: [i32; 2] = [1, 30];
const DEFAULT_GOP: i32 = i32::MAX;
const DEFAULT_HW_QUALITY: Quality = Quality_Default;
const DEFAULT_RC: RateControl = RC_DEFAULT;

pub struct HwEncoder {
    encoder: Encoder,
    yuv: Vec<u8>,
    pub format: DataFormat,
    pub pixfmt: AVPixelFormat,
}

impl EncoderApi for HwEncoder {
    fn new(cfg: EncoderCfg) -> ResultType<Self>
    where
        Self: Sized,
    {
        match cfg {
            EncoderCfg::HW(config) => {
                let ctx = EncodeContext {
                    name: config.name.clone(),
                    width: config.width as _,
                    height: config.height as _,
                    pixfmt: DEFAULT_PIXFMT,
                    align: HW_STRIDE_ALIGN as _,
                    bitrate: config.bitrate * 1000,
                    timebase: DEFAULT_TIME_BASE,
                    gop: DEFAULT_GOP,
                    quality: DEFAULT_HW_QUALITY,
                    rc: DEFAULT_RC,
                };
                let format = match Encoder::format_from_name(config.name.clone()) {
                    Ok(format) => format,
                    Err(_) => {
                        return Err(anyhow!(format!(
                            "failed to get format from name:{}",
                            config.name
                        )))
                    }
                };
                match Encoder::new(ctx.clone()) {
                    Ok(encoder) => Ok(HwEncoder {
                        encoder,
                        yuv: vec![],
                        format,
                        pixfmt: ctx.pixfmt,
                    }),
                    Err(_) => Err(anyhow!(format!("Failed to create encoder"))),
                }
            }
            _ => Err(anyhow!("encoder type mismatch")),
        }
    }

    fn encode_to_message(
        &mut self,
        frame: &[u8],
        _ms: i64,
    ) -> ResultType<hbb_common::message_proto::Message> {
        let mut msg_out = Message::new();
        let mut vf = VideoFrame::new();
        let mut frames = Vec::new();
        for frame in self.encode(frame).with_context(|| "Failed to encode")? {
            frames.push(EncodedVideoFrame {
                data: Bytes::from(frame.data),
                pts: frame.pts as _,
                key: frame.key == 1,
                ..Default::default()
            });
        }
        if frames.len() > 0 {
            let frames = EncodedVideoFrames {
                frames: frames.into(),
                ..Default::default()
            };
            match self.format {
                DataFormat::H264 => vf.set_h264s(frames),
                DataFormat::H265 => vf.set_h265s(frames),
            }
            msg_out.set_video_frame(vf);
            Ok(msg_out)
        } else {
            Err(anyhow!("no valid frame"))
        }
    }

    fn use_yuv(&self) -> bool {
        false
    }

    fn set_bitrate(&mut self, bitrate: u32) -> ResultType<()> {
        self.encoder.set_bitrate((bitrate * 1000) as _).ok();
        Ok(())
    }
}

impl HwEncoder {
    pub fn best() -> CodecInfos {
        get_config(CFG_KEY_ENCODER).unwrap_or(CodecInfos {
            h264: None,
            h265: None,
        })
    }

    pub fn encode(&mut self, bgra: &[u8]) -> ResultType<Vec<EncodeFrame>> {
        match self.pixfmt {
            AVPixelFormat::AV_PIX_FMT_YUV420P => hw::hw_bgra_to_i420(
                self.encoder.ctx.width as _,
                self.encoder.ctx.height as _,
                &self.encoder.linesize,
                &self.encoder.offset,
                self.encoder.length,
                bgra,
                &mut self.yuv,
            ),
            AVPixelFormat::AV_PIX_FMT_NV12 => hw::hw_bgra_to_nv12(
                self.encoder.ctx.width as _,
                self.encoder.ctx.height as _,
                &self.encoder.linesize,
                &self.encoder.offset,
                self.encoder.length,
                bgra,
                &mut self.yuv,
            ),
        }

        match self.encoder.encode(&self.yuv) {
            Ok(v) => {
                let mut data = Vec::<EncodeFrame>::new();
                data.append(v);
                Ok(data)
            }
            Err(_) => Ok(Vec::<EncodeFrame>::new()),
        }
    }
}

pub struct HwDecoder {
    decoder: Decoder,
    pub info: CodecInfo,
}

#[derive(Default)]
pub struct HwDecoders {
    pub h264: Option<HwDecoder>,
    pub h265: Option<HwDecoder>,
}

impl HwDecoder {
    pub fn best() -> CodecInfos {
        get_config(CFG_KEY_DECODER).unwrap_or(CodecInfos {
            h264: None,
            h265: None,
        })
    }

    pub fn new_decoders() -> HwDecoders {
        let best = HwDecoder::best();
        let mut h264: Option<HwDecoder> = None;
        let mut h265: Option<HwDecoder> = None;
        let mut fail = false;

        if let Some(info) = best.h264 {
            h264 = HwDecoder::new(info).ok();
            if h264.is_none() {
                fail = true;
            }
        }
        if let Some(info) = best.h265 {
            h265 = HwDecoder::new(info).ok();
            if h265.is_none() {
                fail = true;
            }
        }
        if fail {
            check_config_process();
        }
        HwDecoders { h264, h265 }
    }

    pub fn new(info: CodecInfo) -> ResultType<Self> {
        let ctx = DecodeContext {
            name: info.name.clone(),
            device_type: info.hwdevice.clone(),
        };
        match Decoder::new(ctx) {
            Ok(decoder) => Ok(HwDecoder { decoder, info }),
            Err(_) => Err(anyhow!(format!("Failed to create decoder"))),
        }
    }
    pub fn decode(&mut self, data: &[u8]) -> ResultType<Vec<HwDecoderImage>> {
        match self.decoder.decode(data) {
            Ok(v) => Ok(v.iter().map(|f| HwDecoderImage { frame: f }).collect()),
            Err(_) => Ok(vec![]),
        }
    }
}

pub struct HwDecoderImage<'a> {
    frame: &'a DecodeFrame,
}

impl HwDecoderImage<'_> {
    // take dst_stride into account when you convert
    pub fn to_fmt(
        &self,
        (fmt, dst_stride): (ImageFormat, usize),
        fmt_data: &mut Vec<u8>,
        i420: &mut Vec<u8>,
    ) -> ResultType<()> {
        let frame = self.frame;
        match frame.pixfmt {
            AVPixelFormat::AV_PIX_FMT_NV12 => hw::hw_nv12_to(
                fmt,
                frame.width as _,
                frame.height as _,
                &frame.data[0],
                &frame.data[1],
                frame.linesize[0] as _,
                frame.linesize[1] as _,
                fmt_data,
                i420,
                HW_STRIDE_ALIGN,
            ),
            AVPixelFormat::AV_PIX_FMT_YUV420P => {
                hw::hw_i420_to(
                    fmt,
                    frame.width as _,
                    frame.height as _,
                    &frame.data[0],
                    &frame.data[1],
                    &frame.data[2],
                    frame.linesize[0] as _,
                    frame.linesize[1] as _,
                    frame.linesize[2] as _,
                    fmt_data,
                );
                return Ok(());
            }
        }
    }

    pub fn bgra(&self, bgra: &mut Vec<u8>, i420: &mut Vec<u8>) -> ResultType<()> {
        self.to_fmt((ImageFormat::ARGB, 1), bgra, i420)
    }

    pub fn rgba(&self, rgba: &mut Vec<u8>, i420: &mut Vec<u8>) -> ResultType<()> {
        self.to_fmt((ImageFormat::ABGR, 1), rgba, i420)
    }
}

fn get_config(k: &str) -> ResultType<CodecInfos> {
    let v = HwCodecConfig::get()
        .options
        .get(k)
        .unwrap_or(&"".to_owned())
        .to_owned();
    match CodecInfos::deserialize(&v) {
        Ok(v) => Ok(v),
        Err(_) => Err(anyhow!("Failed to get config:{}", k)),
    }
}

pub fn check_config() {
    let ctx = EncodeContext {
        name: String::from(""),
        width: 1920,
        height: 1080,
        pixfmt: DEFAULT_PIXFMT,
        align: HW_STRIDE_ALIGN as _,
        bitrate: 0,
        timebase: DEFAULT_TIME_BASE,
        gop: DEFAULT_GOP,
        quality: DEFAULT_HW_QUALITY,
        rc: DEFAULT_RC,
    };
    let encoders = CodecInfo::score(Encoder::available_encoders(ctx));
    let decoders = CodecInfo::score(Decoder::available_decoders());

    if let Ok(old_encoders) = get_config(CFG_KEY_ENCODER) {
        if let Ok(old_decoders) = get_config(CFG_KEY_DECODER) {
            if encoders == old_encoders && decoders == old_decoders {
                return;
            }
        }
    }

    if let Ok(encoders) = encoders.serialize() {
        if let Ok(decoders) = decoders.serialize() {
            let mut config = HwCodecConfig::load();
            config.options.insert(CFG_KEY_ENCODER.to_owned(), encoders);
            config.options.insert(CFG_KEY_DECODER.to_owned(), decoders);
            config.store();
            return;
        }
    }
    log::error!("Failed to serialize codec info");
}

pub fn check_config_process() {
    use hbb_common::sysinfo::{ProcessExt, System, SystemExt};

    std::thread::spawn(move || {
        HwCodecConfig::remove();
        if let Ok(exe) = std::env::current_exe() {
            if let Some(file_name) = exe.file_name().to_owned() {
                let s = System::new_all();
                let arg = "--check-hwcodec-config";
                for process in s.processes_by_name(&file_name.to_string_lossy().to_string()) {
                    if process.cmd().iter().any(|cmd| cmd.contains(arg)) {
                        log::warn!("already have process {}", arg);
                        return;
                    }
                }
                if let Ok(mut child) = std::process::Command::new(exe).arg(arg).spawn() {
                    let second = 3;
                    std::thread::sleep(std::time::Duration::from_secs(second));
                    // kill: Different platforms have different results
                    allow_err!(child.kill());
                    std::thread::sleep(std::time::Duration::from_millis(30));
                    match child.try_wait() {
                        Ok(Some(status)) => log::info!("Check hwcodec config, exit with: {status}"),
                        Ok(None) => {
                            log::info!(
                                "Check hwcodec config, status not ready yet, let's really wait"
                            );
                            let res = child.wait();
                            log::info!("Check hwcodec config, wait result: {res:?}");
                        }
                        Err(e) => {
                            log::error!("Check hwcodec config, error attempting to wait: {e}")
                        }
                    }
                    HwCodecConfig::refresh();
                }
            }
        };
    });
}
