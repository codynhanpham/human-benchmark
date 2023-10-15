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

pub fn reaction_time_inject() {
    let js_script = r#"
let count = 0;
function reactionClick() {
    if (count > 11) { console.log('Done!'); return; };
    
    const clickArea = document.querySelector('div[data-test="true"]');
    const color = window.getComputedStyle(clickArea).background;
    if (color.startsWith('rgb(75, 219, 106)') || color.startsWith('rgb(43, 135, 209)')) {
        count += 1;
        const box = clickArea.getBoundingClientRect(),
        coordX = box.left + (box.right - box.left) / 2,
        coordY = box.top + (box.bottom - box.top) / 2;
        simulateMouseEvent(clickArea, "mousedown", coordX, coordY);
        simulateMouseEvent(clickArea, "mouseup", coordX, coordY);
        simulateMouseEvent(clickArea, "click", coordX, coordY);
    }
    window.requestAnimationFrame(reactionClick);
}

window.requestAnimationFrame(reactionClick);

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