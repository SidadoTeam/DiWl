static mut app_style: String = String::new();

pub fn init_app_style() {
    let sty = stylist::style!(
        r#"
        html,
        body {
          margin: 0;
          color:white;
          background-color: rgb(68, 55, 55);
        }
        .card{
          position: fixed;
            height: 300px;
            width: 300px;
            top: 90px;
            right: 50px;
            background: #8f7b7b;
    
            border-radius: 5px;
            box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2);
            padding-left: 20px;
            padding-right: 20px;
            padding-top: 10px;
        }
        .diwl-button{
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
        .bg-color-gray{
          background-color: gray;
        }
    
        .bg-color-black{
          background-color: rgb(83, 81, 81);
        }
    
        .align-buttom-10{
          position: absolute;
          bottom: 10px;
          right: 10px;
        }
        .align-buttom-20{
          position: absolute;
          bottom: 20px;
          right: 20px;
        }
        .color-white-1{
          color: rgb(235, 227, 227);
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
