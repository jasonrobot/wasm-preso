use yew::prelude::*;

mod video_list;
use video_list::Video;
use video_list::VideosList;

// let videos: Vec<Video> = vec![
//     Video {
//         id: 1,
//         title: "Test".to_string(),
//         view_count: 420,
//         url: "https://www.youtube.com/watch?v=BXan_z7ePBI".to_string(),
//     },
// ];

fn get_videos() -> Vec<Video> {
    vec![
        Video {
            id: 1,
            title: "Test".to_string(),
            view_count: 420,
            url: "https://www.youtube.com/watch?v=BXan_z7ePBI".to_string(),
        },
    ]
}

#[function_component(App)]
fn app() -> Html {


    html! {
        <>
            <h1>{ "Hello, world!" }</h1>
            <VideosList videos={get_videos()}/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
