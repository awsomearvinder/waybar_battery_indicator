pub fn background_color(color :&str, input :&str) -> String{
    format!("<span background = '{color}'> {text} </span>", color=color, text=input)
}