(() => {
    function simulateMouseEvent(element, eventName, coordX, coordY) {
        element.dispatchEvent(new MouseEvent(eventName, {
            view: window,
            bubbles: true,
            cancelable: true,
            clientX: coordX,
            clientY: coordY,
            button: 0
        }));
    };
    function clickSim(element) {
        const box = element.getBoundingClientRect(),
            coordX = box.left + (box.right - box.left) / 2,
            coordY = box.top + (box.bottom - box.top) / 2;
        simulateMouseEvent(element, "mousedown", coordX, coordY);
        simulateMouseEvent(element, "mouseup", coordX, coordY);
        simulateMouseEvent(element, "click", coordX, coordY);
        delete box, coordX, coordY;
    }
    function restart() {
        const restartButton = document.querySelector('div[data-test="true"] button.secondary');
        if (restartButton) {
            restartButton.click();
        }
    }
    restart();
    let count = 0;
    function aimClick() {
        if (count > 31) return;

        count +=1;
        const aimTrue = document.querySelector('div[data-aim-target="true"]');
        const targetDiv = aimTrue?.getElementsByTagName('div')[3] || null;
        if (targetDiv == null) return;

        clickSim(targetDiv);

        window.requestAnimationFrame(aimClick);
    }

    window.requestAnimationFrame(aimClick);
})();