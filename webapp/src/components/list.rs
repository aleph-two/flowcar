use leptos::*;

use crate::components::card::Card;
use crate::models::vehicle::Vehicle;

#[component]
pub fn List(vehicles: Vec<Vehicle>) -> impl IntoView {
    view! {
        <div class="container-fluid container-lg my-3">
            <div class="row row-cols-1 g-3">
                {vehicles
                    .iter()
                    .map(|vehicle| {
                        view! {
                            <div class="col">
                                <Card vehicle=vehicle.clone()/>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </div>
        </div>
    }
}
