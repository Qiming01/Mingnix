use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use tokio::{fs, io::AsyncWriteExt, net::TcpStream};

use crate::{
    http::{
        request::Request,
        response::{ContentType, HttpStatus, Response},
    },
    SharedData,
};

pub struct Index;
pub struct NotFound;
pub struct VisitCount;
pub struct Echo<'a> {
    pub path_buf: &'a [u8],
}
pub struct StaticFile<'a> {
    pub path_buf: &'a [u8],
}

#[async_trait]
pub trait Handler {
    async fn handle(&self, stream: &mut TcpStream, shared_data: Arc<Mutex<SharedData>>);
}

#[async_trait]
impl Handler for Index {
    async fn handle(&self, stream: &mut TcpStream, _shared_data: Arc<Mutex<SharedData>>) {
        let body = r#"<!DOCTYPE html><html lang='en'><head><meta charset='UTF-8'><title>Minginx</title><meta name='viewport'content='width=device-width, initial-scale=1, user-scalable=no'><meta name='description'content='Lorem ipsum dolor sit amet, consectetur adipisicing elit. Laudantium illum ullam accusantium maxime veritatis.'><link href='http://fonts.googleapis.com/css?family=Open+Sans:300,400,900'rel='stylesheet'type='text/css'><link rel='stylesheet'href='static/index.css'></head><body><div class='spinner'><div class='bounce1'></div><div class='bounce2'></div><div class='bounce3'></div></div><section class='error'><h1>Minginx</h1></section><div id='svg_container'><svg xmlns='http://www.w3.org/2000/svg'xmlns:xlink='http://www.w3.org/1999/xlink'id='submarine_container'viewBox='0 0 1366 768'version='1.1'><title>Submarine</title><desc>Minginx</desc><g id='Page-1'stroke='none'stroke-width='1'fill='none'fill-rule='evenodd'><g id='canvas'><g id='Submarine'transform='translate(414.000000, 214.000000)'><g id='body'><g id='Periscope'transform='translate(238.000000, 0.000000)'><rect id='Rectangle-11'fill='#E26A6F'x='0'y='5'width='18'height='81'/><path d='M0 5 L53.9457932 0.2 L53.9457932 27.8 L0 24 L0 5 Z'id='Rectangle-12'fill='#F18085'/></g><path d='M68.4512504 192.7 L68.2214491 192.7 L42.4574319 158.7 L39.1877254 161.1 L44.3458985 167.9 L64.7346837 194.7 C64.7346837 194.7 61 199 55 203 C38.9394756 213.8 16.2 208.1 6.4 193.6 C-3.43610944 179 -0.2 155.8 15.8 144.9 L23.2865152 139.9 L31.0436693 150.2 L31.0436693 150.2 L34.2650816 147.9 L34.2650816 147.9 L26.1522911 137.2 L42.4360184 131.9 L61.0237487 125.6 L61.1948172 125.9 L86.3859435 117.8 C86.3859435 117.8 129.5 105.8 183.6 93.5 C180.722922 79.4 179.9 62.4 189.8 55.4 C208.11679 42.4 301.6 42.2 322.5 55.4 C327.18244 58.3 329.8 62.9 331 68.4 C339.308255 67.8 347.1 67.5 354.2 67.5 C481.773695 68.8 506.4 124 496.8 165.4 C487.220548 206.7 441.9 235.6 354.2 234.7 C316.838342 234.3 139.5 205.2 139.5 205.2 L113.634294 200.6 L113.672949 200.7 L68.464691 192.7 L68.4512504 192.7 Z'id='Submarine_body'fill='#F18085'/><path d='M495.635398 164.7 C485.494798 205.5 440.2 233.8 353.3 232.9 C315.916933 232.5 138.6 203.4 138.6 203.4 L112.712885 198.9 L112.712885 198.9 L112.751539 199 L67.5432818 191 L67.5298412 191 L67.3000398 190.9 L47.4443967 164.7 L495.635398 164.7 Z'id='submarine_body_shade'fill='#E26A6F'/><g id='lights'transform='translate(251.737184, 73.224560) scale(-1, 1) translate(-251.737184, -73.224560) translate(204.237184, 61.224560)'><rect id='light_container'fill='#E26A6F'x='0'y='0'width='94.9'height='24'rx='8'/><ellipse class='lights'id='light_right'fill='#c9c9c9'cx='16.6'cy='11.9'rx='4.6'ry='4.6'/><ellipse class='lights'id='light_right_first_left'fill='#c9c9c9'cx='31.3'cy='11.9'rx='4.6'ry='4.6'/><ellipse class='lights'id='light_center'fill='#c9c9c9'cx='46.1'cy='11.9'rx='4.6'ry='4.6'/><ellipse class='lights'id='light_left_next_right'fill='#c9c9c9'cx='61.7'cy='11.9'rx='4.6'ry='4.6'/><ellipse class='lights'id='light_left'fill='#c9c9c9'cx='77.4'cy='11.9'rx='4.6'ry='4.6'/></g><g id='head_light'><path d='M489.1 232.9 L505.5 206.7 L3699.7 1066.7 L2166 2791.6 L489.1 232.9 Z'id='light'fill='#8E9EB7'/><path d='M484 230.8 L490.2 234.2 L498 220.2 L505.8 206.3 L499.6 202.9 C488.2 196.6 475.4 197.8 471.1 205.5 C466.8 213.2 472.6 224.5 484 230.8 L484 230.8 Z'id='light_emitter'fill='#F67C81'/></g><g id='window'transform='translate(422.974221, 151.801013) rotate(105.000000) translate(-422.974221, -151.801013) translate(364.474221, 93.301013)'><ellipse id='glass'stroke='#B1B1B1'stroke-width='8'fill='#4D576D'cx='58.5'cy='58.4'rx='57.6'ry='57.6'/><path d='M58.9701897 4.6 C58.6758133 4.6 58.4 4.6 58.1 4.6 C28.5500958 4.6 4.6 28.8 4.6 58.5 C4.60704607 88.3 28.6 112.5 58.1 112.5 C58.3808588 112.5 58.7 112.4 59 112.4 L58.9701897 4.6 Z'id='glass_shade'fill='#414B5F'/></g><g id='fan_container'transform='translate(203.000000, 130.000000)'><ellipse id='container'stroke='#FF868B'stroke-width='7'fill='#EF7378'cx='34.3'cy='34.3'rx='33.6'ry='33.6'/><path d='M41.5386329 37.3 L63.5624248 28.5 L59.3750415 18.3 L37.3512496 27.1 L28.4530602 5.3 L18.0889228 9.4 L26.9871123 31.2 L4.96332048 40 L9.15070377 50.3 L31.1744956 41.5 L40.0726851 63.3 L50.4368224 59.1 L41.5386329 37.3 L41.5386329 37.3 Z'id='fan'fill='#FFFFFF'/></g></g></g><g id='bubble_group'><circle id='bubble_medium'fill='#73849E'cx='343.5'cy='398.5'r='14.5'/><ellipse id='bubble_large'fill='#73849E'cx='252.5'cy='349.5'rx='20.5'ry='18.5'/><circle id='bubble_small'fill='#73849E'cx='413.5'cy='349.5'r='10.5'/></g></g></g></svg><svg id='waves'width='2000'viewBox='0 0 1366 768'><g><path d='M0 583 C0 583 101.7 533.5 150.5 533.5 C238.588591 533.5 248.9 583 336.9 583 C382.922767 583 446.1 533.5 492.1 533.5 C560.583282 533.5 611.8 583 680.2 583 C750.322273 583 796.1 533.5 866.2 533.5 C933.203386 533.5 1024.5 583 1091.5 583 C1126.65335 583 1186.8 533.5 1222 533.5 C1278.314 533.5 1366 583 1366 583 L1366 768 L0 768 L0 583 Z'id='wave_bottom'fill-opacity='0.23680933'fill='#3E4A5D'/><path d='M0 653 C0 653 115.4 603.5 170.7 603.5 C270.551778 603.5 282.2 653 382.1 653 C434.222084 653 505.9 603.5 558.1 603.5 C635.683385 603.5 693.8 653 771.4 653 C850.841289 653 902.8 603.5 982.2 603.5 C1058.22258 603.5 1161.8 653 1237.8 653 C1277.58861 653 1345.8 603.5 1385.7 603.5 C1449.5669 603.5 1549 653 1549 653 L1549 838 L0 838 L0 653 Z'id='wave_top'fill-opacity='0.23680933'fill='#3E4A5D'/></g></svg></div><script type='text/javascript'src='static/jquery.min.js'></script><script type='text/javascript'src='static/code.js'></script></body></html>"#;

        let mut response = Response::new();
        let response = response
            .set_status(HttpStatus::Ok)
            .set_headers("Content-Type".into(), ContentType::Html.to_string())
            .set_headers("Content-Length".into(), body.len().to_string())
            .set_body(body.as_bytes());

        stream.write_all(&response.as_bytes()).await.unwrap();
        stream.flush().await.unwrap();
    }
}

#[async_trait]
impl Handler for VisitCount {
    async fn handle(&self, stream: &mut TcpStream, shared_data: Arc<Mutex<SharedData>>) {
        shared_data.lock().unwrap().visit_count += 1;
        let visit_count = shared_data.lock().unwrap().visit_count;

        let body = format!("{} Times!", visit_count);

        let mut response = Response::new();
        let response = response
            .set_status(HttpStatus::Ok)
            .set_headers("Content-Type".into(), ContentType::Html.to_string())
            .set_headers("Content-Length".into(), body.len().to_string())
            .set_body(body.as_bytes());

        stream.write_all(&response.as_bytes()).await.unwrap();
        stream.flush().await.unwrap();
    }
}

#[async_trait]
impl Handler for Echo<'_> {
    async fn handle(&self, stream: &mut TcpStream, _shared_data: Arc<Mutex<SharedData>>) {
        let req: Request = self.path_buf.to_vec().into();
        let queries = req.parse_queries();

        let body = queries.get("content").unwrap_or(&"Need some arguments");

        let mut response = Response::new();
        let response = response
            .set_status(HttpStatus::Ok)
            .set_headers("Content-Type".into(), ContentType::Html.to_string())
            .set_headers("Content-Length".into(), body.len().to_string())
            .set_body(body.as_bytes());

        stream.write_all(&response.as_bytes()).await.unwrap();
        stream.flush().await.unwrap();
    }
}

#[async_trait]
impl Handler for StaticFile<'_> {
    async fn handle(&self, stream: &mut TcpStream, shared_data: Arc<Mutex<SharedData>>) {
        // Static file path
        // Read the path from request (struct)
        let buf = &String::from_utf8_lossy(self.path_buf); // Buffer to string

        // GET /static/...
        for i in buf.split_whitespace() {
            if i.contains("/static/") {
                let path = i.split('/').collect::<Vec<&str>>()[2]; // static/<path>
                let file = fs::read(format!("static/{path}")).await;

                // 也可以直接使用URL里的 `static/<path>`，不过像上面这样写可以用在当静态文件的文件夹不是 `static` 的时候
                // let path = i.strip_prefix('/').unwrap_or_default();
                // let file = fs::read(path).await;

                if let Ok(f) = file {
                    let content_type = parse_content_type(path);

                    let mut response = Response::new();
                    let response = response
                        .set_status(HttpStatus::Ok)
                        .set_headers("Content-Type".into(), content_type.to_string())
                        .set_headers("Content-Length".into(), f.len().to_string())
                        .set_body(&f);

                    stream.write_all(&response.as_bytes()).await.unwrap();
                    stream.flush().await.unwrap();
                } else {
                    NotFound.handle(stream, Arc::clone(&shared_data)).await;
                }
            }
        }
    }
}

#[async_trait]
impl Handler for NotFound {
    async fn handle(&self, stream: &mut TcpStream, _shared_data: Arc<Mutex<SharedData>>) {
        let body = r#"<!doctype html><html><head><meta charset="utf-8"><title>404</title><style>html{margin:0;padding:0;background-color:white}body,html{width:100%;height:100%;overflow:hidden}#svgContainer{width:640px;height:512px;background-color:white;position:absolute;top:0;left:0;right:0;bottom:0;margin:auto}</style></head><body><script type="text/javascript"src="static/bodymovin.js"></script><script type="text/javascript"src="static/data.js"></script><div id="svgContainer"></div><script type="text/javascript">var svgContainer=document.getElementById("svgContainer");var animItem=bodymovin.loadAnimation({wrapper:svgContainer,animType:"svg",loop:true,animationData:JSON.parse(animationData)});</script></body></html>"#;

        let mut response = Response::new();
        let response = response
            .set_status(HttpStatus::NotFound)
            .set_headers("Content-Type".into(), ContentType::Html.to_string())
            .set_headers("Content-Length".into(), body.len().to_string())
            .set_body(body.as_bytes());

        stream.write_all(&response.as_bytes()).await.unwrap();
        stream.flush().await.unwrap();
    }
}
// Parse the `Content-Type` from request
fn parse_content_type(req: &str) -> ContentType {
    // .html or .htm
    if req.contains(".htm") {
        ContentType::Html
    } else if req.contains(".txt") {
        ContentType::PlainText
    } else if req.contains(".css") {
        ContentType::Css
    } else if req.contains(".png") || req.contains(".jpg") || req.contains(".ico") {
        ContentType::AvifImage
    } else if req.contains(".mp4") {
        ContentType::Mp4Video
    } else {
        ContentType::Html
    }
}
