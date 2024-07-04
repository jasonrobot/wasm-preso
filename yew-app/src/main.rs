mod video_list;
use video_list::Video;
use video_list::VideosList;
mod video_details;
use video_details::VideoDetails;
use yew::prelude::*;

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
    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            if *selected_video == Some(video.clone()) {
                selected_video.set(None);
            } else {
                selected_video.set(Some(video))
            }
        })
    };

    let details = selected_video.as_ref().map(|video| html! {
        <VideoDetails video={video.clone()} />
    });

    html! {
        <>
            <h1>{ "Video watch list:" }</h1>
            <VideosList videos={get_videos()} on_click={on_video_select.clone()}/>
            if details.is_some() {
                <div>{ "Video details:" }</div>
            }
            { for details }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
