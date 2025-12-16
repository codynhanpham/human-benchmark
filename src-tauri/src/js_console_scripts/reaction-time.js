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
    
    let count = 0;
    const clickArea = document.querySelector('div[data-test="true"]');
    function reactionClick() {
        if (count > 11) { console.log('Done!'); return; };
        
        const color = window.getComputedStyle(clickArea).background;
        if (color.startsWith('rgb(75, 219, 106)') || color.startsWith('rgb(43, 135, 209)')) {
            count += 1;
            clickSim(clickArea);
        }
        window.requestAnimationFrame(reactionClick);
    }
    window.requestAnimationFrame(reactionClick);
})();