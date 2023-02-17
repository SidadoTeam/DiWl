static mut app_style: String = String::new();

pub fn init_app_style() {
    let sty = stylist::style!(
        r#"
        html,
    dialog {
      margin: 0;
      color: white;
      background-color: rgb(68, 55, 55);
    }
    z-index:4000;

    .size-15 {
      size: 15px;
    }

    .font-size-15 {
      font-size: 15px;
    }

    .text-size-15 {
      font-size: 15px;
    }

    .button {
      cursor: pointer;
      border: 1px solid transparent;
      outline: none;
      background-color: transparent;
    }
    
    .mcard {
      color: white;
      background-color: rgb(68, 55, 55);
      position: fixed;
      height: 300px;
      width: 300px;
      top: 250px;
      right: 350px;
      background: black;
      border-radius: 5px;
      box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2);
      padding: 10px;
    }
    
    .word-card{
      margin-top: 10px;
      padding: 10px;
      border-radius: 5px;
      background-color: rgb(30, 36, 30);
    }

    .opx-80 {
      opacity: 80%;
    }

    .opx-90 {
      opacity: 90%;
    }
    .diwl-button {
      padding-left: 15px;
      padding-right: 15px;
      padding-top: 6px;
      padding-bottom: 6px;
      background-color: green;
      border-radius: 5px;
      font-size: 14px;
      align-content: center;
      align-items: center;
      color: white;
    }

    .bg-color-gray {
      background-color: gray;
    }
    
    .bg-color-black1 {
      background-color: rgb(30, 36, 30);
    }

    .bg-color-black {
      background-color: rgb(83, 81, 81);
    }

    .align-buttom-right {
      position: absolute;
      bottom: 10px;
      right: 20px;
    }

    .padding-5 {
      padding: 5px;
    }

    .padding-word {
      padding-left: 3px;
      padding-right: 3px;
      border-radius: 3px;
    }

    .margin-5 {
      margin: 5px;
    }

    .color-white-1 {
      color: rgb(235, 227, 227);
    }

    .color-yell{
      color:yellowgreen;
    }
    "#
    );
    unsafe {
        app_style = sty.unwrap().get_class_name().to_owned();
    }
}

pub fn get_app_style() -> String {
    unsafe { app_style.clone() }
}
