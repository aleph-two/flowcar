use leptos::*;
use leptos_router::{use_params_map, use_query_map};

use crate::components::carousel::Carousel;
use crate::components::footer::Footer;
use crate::components::nav::NavBar;
use crate::components::summary::Summary;
use crate::models::model::Model;
use crate::models::vehicle::Vehicle;

#[server]
pub async fn get_vehicle(id: Option<String>) -> Result<(Vehicle, Model), ServerFnError> {
    use crate::database;

    let pool = database::db().await.unwrap();

    let vehicle_query = "SELECT * FROM vehicles WHERE id = $1::uuid";
    let vehicle = sqlx::query_as::<_, Vehicle>(vehicle_query)
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();

    let model_query = "SELECT * FROM models WHERE id = $1::uuid";
    let model = sqlx::query_as::<_, Model>(model_query)
        .bind(vehicle.model)
        .fetch_one(&pool)
        .await
        .unwrap();

    Ok((vehicle, model))
}

#[component]
pub fn Vehicle() -> impl IntoView {
    let params = use_params_map();
    let _query = use_query_map();

    #[derive(Clone, PartialEq)]
    struct QueryInfo {
        id: Option<String>,
    }

    let query_info = move || {
        with!(|params, _query| QueryInfo {
            id: params.get("id").cloned(),
        })
    };

    let vehicle_resource = create_resource(query_info, move |info| get_vehicle(info.id));

    view! {
        <NavBar/>
        <Suspense>
        {move || match vehicle_resource.get() {
            Some(Ok((vehicle, _model))) => view! {
                <>
                    <div class="container-lg conditional-shadow my-lg-5">
                        <div class="row">
                            <div class="col-12 col-lg-3 col-md-6 p-0">
                                <div class="ratio ratio-9x16">
                                    <video src=&vehicle.video.src alt=&vehicle.video.alt autoplay loop muted></video>
                                </div>
                            </div>
                            <div class="col-12 col-lg-9 col-md-6 p-4">
                                <Summary vehicle=vehicle.clone()/>
                            </div>
                        </div>
                    </div>
                    <Carousel vehicle=vehicle.clone()/>
                </>
            },
            Some(_) | None => view! { <>{}</> },
        }}
        </Suspense>
        <Footer/>
    }
}
