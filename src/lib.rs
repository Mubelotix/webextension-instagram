use wasm_bindgen::{prelude::*, JsCast};
use web_sys::console;
use web_sys::HtmlElement;
use web_sys::Event;
use js_sys::Object;
use web_sys::XPathNsResolver;
use std::panic;
use std::rc::Rc;
use web_sys::HtmlDivElement;
use web_sys::Storage;

const MAX_LIKE: usize = 40;

fn get_element_by_xpath(xpath: &str) -> Result<Option<HtmlElement>, &'static str> {
    let document = web_sys::window().expect("should have a window").document().expect("should have a document in the window");
    if let Ok(xpathresult) = document.evaluate_with_opt_x_path_ns_resolver_and_type_and_result(xpath, &document, Some(&XPathNsResolver::new()), 9, Some(&Object::new())) {
        if let Ok(result) = xpathresult.single_node_value() {
            if let Some(element) = result {
                if let Ok(button) = element.dyn_into::<web_sys::HtmlElement>() {
                    Ok(Some(button))
                } else {
                    Err("can't cast element")
                }
            } else {
                Ok(None)
            }
        } else {
            Err("can't get response of xpath search")
        }
    } else {
        Err("can't search element with xpath")
    }
}

fn get_element_by_tag_name(tag_name: &str) -> Result<Option<HtmlElement>, &'static str> {
    let document = web_sys::window().expect("should have a window").document().expect("should have a document in the window");
    let result = document.get_elements_by_tag_name(tag_name);
    if let Some(element) = result.get_with_index(0) {
        if let Ok(button) = element.dyn_into::<web_sys::HtmlElement>() {
            Ok(Some(button))
        } else {
            Err("can't cast element")
        }
    } else {
        Ok(None)
    }
}

fn go_to_profile() -> Result<(), &'static str> {
    if let Some(button) = get_element_by_xpath("/html/body/span/section/nav/div[2]/div/div/div[3]/div/div[3]/a")? {
        button.click();
        Ok(())
    } else {
        Err("can't find profile link")
    }
}
fn get_like_number() -> Result<usize, &'static str> {
    if let Some(counter) = get_element_by_xpath("//button[@class='sqdOP yWX7d     _8A5w5    ']/span")? {
        if let Ok(like_number) = counter.inner_html().parse() {
            Ok(like_number)
        } else {
            Err("can't parse like number")
        }
    } else {
        Err("can't find like counter")
    }
}

fn like() -> Result<(), &'static str> {
    if let Some(button) = get_element_by_xpath("//button[contains(@class,'dCJp8 afkep')]")? {
        button.click();
        Ok(())
    } else {
        Err("can't find heart button")
    }
}

fn next() -> Result<(), &'static str> {
    if let Some(button) = get_element_by_xpath("//a[contains(@class,'HBoOv coreSpriteRightPaginationArrow')]")? {
        button.click();
        Ok(())
    } else {
        Err("can't find the next button")
    }
}

fn test_function(mut remaining: usize, task: HtmlDivElement) {
    task.set_inner_text(&format!("LIKING: {} posts remaining", remaining));

    let closure = Closure::wrap(Box::new(move || {
        let must_be_liked;

        if let Ok(like_number) = get_like_number() {
            if like_number > MAX_LIKE {
                must_be_liked = false;
                console::log_1(&format!("Too much likes ({}/{})", like_number, MAX_LIKE).into());
            } else {
                must_be_liked = true;
            }
        } else if let Err(e) = get_like_number() {
            must_be_liked = true;
            console::warn_1(&format!("Can't get like number ! error: {}", e).into());
        } else {
            must_be_liked = true;
        }

        if must_be_liked {
            if let Err(e) = like() {
                console::warn_1(&format!("Can't like ! error: {}", e).into());
            }
            remaining -= 1;
        }

        if let Err(e) = next() {
            console::warn_1(&format!("Can't move to next post ! error: {}", e).into());
        }
        
        if remaining > 0 {
            test_function(remaining, task.clone());
        } else {
            task.remove();
        }
    }) as Box<dyn FnMut()>);
    let window = web_sys::window().unwrap();
        window.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 5000).unwrap();
    closure.forget();
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console::log_1(&format!("Program started").into());

    panic::set_hook(Box::new(|panic_info| {
        let message: String;
        let file;
        let line;
        let column;
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            message = s.to_string();
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            message = s.to_string();
        } else {
            message = String::from("[unprintable message]")
        }
        if let Some(location) = panic_info.location() {
            file = location.file();
            line = location.line();
            column = location.column();
        } else {
            file = "unknow file";
            line = 0;
            column = 0;
        }
        console::error_1(&format!("program panicked at '{}', {}:{}:{}", message, file, line, column).into());
    }));
    
    let document = web_sys::window().unwrap().document().unwrap();

    //let mut storage = web_sys::window().expect("no window").local_storage().expect("can't get storage").expect("no storage");
    //storage.set_item("test", "test").expect("can't use storage");

    let style_element = Rc::new(document
        .create_element("style").unwrap()
        .dyn_into::<web_sys::HtmlStyleElement>().unwrap());
    style_element.set_inner_text("
        #task_logger {
            all: unset;
            position: fixed;
            top: 20px;
            left: 40vw;
            width: 20vw;
            display: flex,
            flex-direction: column;
            justify-content: center;
            z-index: 999;
            text-align: center;
        }

        #menu {
            all: unset;
            position: fixed;
            bottom: 30px;
            left: 30px;
            width: 27vw;
            background-color: rgba(253, 253, 253, 0.97);
            border-radius: 5px;
            box-shadow: 0 0 3px 3px rgba(0,0,0,0.12);
            z-index: 999;
            text-align: center;
        }

        #title {
            background-color: #000000;
            color: white;
            font-size: large;
            justify-content: center;
            padding: 5px 0 5px 0;
        }

        #flex_box {
            display: flex;
            flex-direction: column;
            justify-content: space-around;
            padding: 10px 15% 0 15%;;
        }

        #flex_box>* {
            margin-bottom: 10px;
        }
    ");
    document.body().unwrap().append_child(&style_element).unwrap();

    let task_logger = Rc::new(document
        .create_element("div").unwrap()
        .dyn_into::<web_sys::HtmlDivElement>().unwrap());
    task_logger.set_id("task_logger");
    document.body().unwrap().append_child(&task_logger).unwrap();
    
    let menu = document
        .create_element("div").unwrap()
        .dyn_into::<web_sys::HtmlDivElement>().unwrap();
    menu.set_id("menu");
    document.body().unwrap().append_child(&menu).unwrap();

    let title = document
        .create_element("div").unwrap()
        .dyn_into::<web_sys::HtmlDivElement>().unwrap();
    title.set_inner_text("Bot control panel!");
    title.set_id("title");
    menu.append_child(&title).unwrap();

    let flex_box = document
        .create_element("div").unwrap()
        .dyn_into::<web_sys::HtmlDivElement>().unwrap();
    flex_box.set_id("flex_box");
    menu.append_child(&flex_box).unwrap();

    let ten_likes_button = document
        .create_element("button").unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>().unwrap();
    ten_likes_button.set_inner_text("Like the 10 following posts");
    flex_box.append_child(&ten_likes_button).unwrap();

    let fifty_likes_button = document
        .create_element("button").unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>().unwrap();
    fifty_likes_button.set_inner_text("Like the 50 following posts!");
    flex_box.append_child(&fifty_likes_button).unwrap();

    let properties = document
        .create_element("button").unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>().unwrap();
    properties.set_inner_text("Properties");
    flex_box.append_child(&properties).unwrap();
    
    let task_logger_clone = Rc::clone(&task_logger);
    let onclick2 = Closure::wrap(Box::new(move |_event: Event| {
        let document = web_sys::window().unwrap().document().unwrap();

        let task = document
            .create_element("div").unwrap()
            .dyn_into::<web_sys::HtmlDivElement>().unwrap();
        task.style().set_property("background-color", "yellow").unwrap();
        task.style().set_property("box-shadow", "0 0 3px 3px rgba(0,0,0,0.12)").unwrap();
        task.style().set_property("margin", "5px 0 5px 0").unwrap();
        task.style().set_property("border-radius", "3px").unwrap();
        task_logger_clone.append_child(&task).unwrap();

        test_function(10, task);
    }) as Box<dyn FnMut(Event)>);
    ten_likes_button
        .add_event_listener_with_callback("click", onclick2.as_ref().unchecked_ref())
        .unwrap();

    let task_logger_clone = Rc::clone(&task_logger);
    let onclick3 = Closure::wrap(Box::new(move |_event: Event| {
        let document = web_sys::window().unwrap().document().unwrap();

        let task = document
            .create_element("div").unwrap()
            .dyn_into::<web_sys::HtmlDivElement>().unwrap();
        task.style().set_property("background-color", "yellow").unwrap();
        task.style().set_property("box-shadow", "0 0 3px 3px rgba(0,0,0,0.12)").unwrap();
        task.style().set_property("margin", "5px 0 5px 0").unwrap();
        task.style().set_property("border-radius", "3px").unwrap();
        task_logger_clone.append_child(&task).unwrap();

        test_function(50, task);
    }) as Box<dyn FnMut(Event)>);
    fifty_likes_button
        .add_event_listener_with_callback("click", onclick3.as_ref().unchecked_ref())
        .unwrap();

    onclick2.forget();
    onclick3.forget();

    // /html/body/span/section/main/div/div[2]/article/div[1]/div/div[1]/div[3]/a
    // /html/body/span/section/main/article/div[2]/div/div[1]/div[1]/a
    Ok(())
}