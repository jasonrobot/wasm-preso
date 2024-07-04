use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub view_count: u64,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>,
}

#[function_component(VideosList)]
pub fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();
    html! {
        <ul>
            {
                videos.iter().map(|video| {
                    let on_video_select = {
                        let on_click = on_click.clone();
                        let video = video.clone();
                        Callback::from(move |_| {
                            on_click.emit(video.clone())
                        })
                    };

                    html! {
                        <li key={video.id} onclick={on_video_select}>
                          {format!("{} ({} views)", video.title, video.view_count)}
                        </li>
                    }
                }).collect::<Html>()
            }
        </ul>
    }
}
