use std::{ops::Deref, rc::Rc, time::Duration};

use html::ElementDescriptor;
use leptos::*;
use leptos_use::{use_timeout_fn, UseTimeoutFnReturn};

use crate::utils::next_frame::use_next_frame;

#[component]
pub fn CssTransition<T>(
    node_ref: NodeRef<T>,
    #[prop(into)] show: MaybeSignal<bool>,
    #[prop(optional)] appear: bool,
    #[prop(into)] enter_from_class: String,
    #[prop(into)] enter_to_class: String,
    #[prop(into)] duration: Duration,
    #[prop(optional, into)] on_after_leave: Option<Callback<()>>,
    children: Children,
) -> impl IntoView
where
    T: Clone + ElementDescriptor + 'static,
{
    let enter_from_class: Rc<[String]> = enter_from_class
        .split_whitespace()
        .map(|val| val.to_owned())
        .collect();

    let enter_to_class: Rc<[String]> = enter_to_class
        .split_whitespace()
        .map(|val| val.to_owned())
        .collect();

    let next_frame = use_next_frame();

    node_ref.on_load(move |node_element| {
        let any_element = node_element.clone().into_any();
        let element = any_element.deref().clone();
        let class_list = element.class_list();

        let run = {
            let class_list = class_list.clone();
            let enter_from_class = Rc::clone(&enter_from_class);
            let enter_to_class = Rc::clone(&enter_to_class);

            Callback::new(move |_: ()| {
                for class in enter_from_class.iter() {
                    class_list.add_1(class).unwrap();
                }

                next_frame.run({
                    let class_list = class_list.clone();
                    let enter_from_class = enter_from_class.clone();
                    let enter_to_class = enter_to_class.clone();

                    move || {
                        for class in enter_to_class.iter() {
                            class_list.add_1(class).unwrap();
                        }

                        for class in enter_from_class.iter() {
                            class_list.remove_1(&class).unwrap();
                        }

                        let UseTimeoutFnReturn { start, stop, .. } = use_timeout_fn(
                            move |_: ()| {
                                for class in enter_to_class.iter() {
                                    class_list.remove_1(&class).unwrap();
                                }
                            },
                            duration.as_secs_f64() * 1000.0,
                        );

                        start(());

                        on_cleanup(move || {
                            stop();
                        })
                    }
                });
            })
        };

        let run_backwards = Callback::new(move |_: ()| {
            for class in enter_to_class.iter() {
                class_list.add_1(class).unwrap();
            }

            next_frame.run({
                let class_list = class_list.clone();
                let enter_from_class = enter_from_class.clone();
                let enter_to_class = enter_to_class.clone();

                move || {
                    for class in enter_from_class.iter() {
                        class_list.add_1(class).unwrap();
                    }

                    for class in enter_to_class.iter() {
                        class_list.remove_1(&class).unwrap();
                    }

                    let UseTimeoutFnReturn {
                        start,
                        stop,
                        is_pending,
                        ..
                    } = use_timeout_fn(
                        move |_: ()| {
                            for class in enter_from_class.iter() {
                                class_list.remove_1(&class).unwrap();
                            }

                            if let Some(on_after_leave) = on_after_leave {
                                on_after_leave(());
                            }
                        },
                        duration.as_secs_f64() * 1000.0,
                    );

                    start(());

                    on_cleanup(move || {
                        if !is_pending.get_untracked() {
                            stop();

                            if let Some(on_after_leave) = on_after_leave {
                                on_after_leave(());
                            }
                        }
                    })
                }
            });
        });

        create_render_effect(move |prev: Option<bool>| {
            let show = show();

            if let Some(prev) = prev {
                if !prev && show {
                    run(());
                } else if prev && !show {
                    run_backwards(());
                }
            } else if appear {
                run(());
            }

            show
        });
    });

    // let (class, set_class) = create_signal("".into());

    // let UseTimeoutFnReturn {
    //     start: start_enter_to,
    //     ..
    // } = use_timeout_fn(
    //     move |_: ()| {
    //         set_class(enter_to_class.clone());
    //     },
    //     duration.as_secs_f64() * 1000.0,
    // );

    // let UseTimeoutFnReturn {
    //     start: start_enter_from,
    //     ..
    // } = use_timeout_fn(
    //     move |_: ()| {
    //         set_class(enter_from_class.clone());
    //         start_enter_to(());
    //     },
    //     duration.as_secs_f64() * 1000.0,
    // );

    // create_render_effect(move |_| {
    //     start_enter_from(());
    // });

    children()
}
