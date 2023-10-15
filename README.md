# Human Benchmark "Helper"

Destroy human benchmark games with this script. That's it.

## Performance
On a ***75 Hz*** refresh rate monitor:
- Typing Test: `> 11,500 WPM`
- Aim Trainer: `< 35 ms`
- Reaction Time: `< 15 ms`
- Sequence Memory: `∞`
- Verbal Memory: `∞`

(Others are work in progress)

## Usage

This script only works with Windows machines.

You can download the Windows executable from the [releases](https://github.com/codynhanpham/human-benchmark/releases) page.

Also, download the English trained data [eng.traineddata](https://github.com/tesseract-ocr/tessdata/blob/main/eng.traineddata) file and place it in the same directory as the executable. This is required for optical character recognition (OCR) tasks.

Since OCR is not perfect, tests requiring OCR may requires a few tries to get a good result.

Launch the executable and follow the instructions.


## JS injection option

To overcome screen refresh rate syncing issues, when selecting the test option, times the number by 10 to show the alternative JavaScript injection code. You can then copy and paste the code into the browser console to run the script.
The JavaScript code uses requestAnimationFrame to sync with the screen refresh rate. On a 75 Hz monitor, the result of the Aim Trainer is consistently 13 ms, exactly the maximum theoretical limit of (1000ms / 75Hz = 13.33ms).

Example:

If you select `2` for the Aim Trainer, input `20` instead.

Here is the result:

```js
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
```