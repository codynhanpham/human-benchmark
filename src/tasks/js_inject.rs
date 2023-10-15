fn print_wrapper(js_script: &str) {
    let instruct = "\n\nOn the test, before hitting start, open the developer console (Ctrl + Shift + J) and paste the following code in the console and hit enter:\n";
    println!("{}", instruct);
    println!("\x1b[33m");
    println!("{}", js_script);
    println!("\x1b[0m");
}

pub fn aim_trainer_inject() {
    let js_script = r#"
    let count = 0;
    function aimClick() {
        if (count > 31) return;

        count +=1;
        const aimTrue = document.querySelector('div[data-aim-target="true"]');
        const targetDiv = aimTrue?.getElementsByTagName('div')[3] || null;
        if (targetDiv == null) return;

        const box = targetDiv.getBoundingClientRect(),
            coordX = box.left + (box.right - box.left) / 2,
            coordY = box.top + (box.bottom - box.top) / 2;
        simulateMouseEvent(targetDiv, "mousedown", coordX, coordY);
        simulateMouseEvent(targetDiv, "mouseup", coordX, coordY);
        simulateMouseEvent(targetDiv, "click", coordX, coordY);

        window.requestAnimationFrame(aimClick);
    }

    window.requestAnimationFrame(aimClick);

    const simulateMouseEvent = function(element, eventName, coordX, coordY) {
        element.dispatchEvent(new MouseEvent(eventName, {
            view: window,
            bubbles: true,
            cancelable: true,
            clientX: coordX,
            clientY: coordY,
            button: 0
        }));
    };
    "#;
    print_wrapper(js_script);
}