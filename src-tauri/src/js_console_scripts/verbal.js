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

const startDiv = document.querySelector('.anim-slide-fade-in')?.getElementsByTagName('div')[0]?.getElementsByTagName('div')[3]?.getElementsByTagName('button')[0] || null;
if (!startDiv) {
    throw new Error("Start button not found.");
}
clickSim(startDiv);
delete startDiv;

const wordList = new Set();
let count = 0;
let errorCount = 0;
function verbalMem() {
    const word = document.querySelector('.word').innerText || null;
    const buttonContainer = document.querySelector('.anim-slide-fade-in')?.getElementsByTagName('div')[0]?.getElementsByTagName('div')[3] || null;
    if (!buttonContainer) {
        throw new Error("Button container not found.");
    }
    const seenButton = buttonContainer.getElementsByTagName('button')[0] || null;
    const newButton = buttonContainer.getElementsByTagName('button')[1] || null;
    if (!seenButton || !newButton) {
        throw new Error("Buttons not found.");
    }

    if (!word) {
        if (errorCount > 2) {
            errorCount = 0;
            console.error("Word not found. This has happened 3 times in a row.");
            console.log('Run the following command to continue:\n');
            console.log('requestAnimationFrame(verbalMem);');
            throw new Error("Word not found. Please run the above command if you want to continue.");
        }
        setTimeout(() => {
            clickSim(seenButton);
            errorCount++;
    
            return verbalMem();
        }, 2000);
    }

    if (wordList.has(word)) {
        clickSim(seenButton);
    } else {
        clickSim(newButton);
        wordList.add(word);
        count++;
        console.log(`Collection size: ${count}\t|\tWord: ${word}`);
    }

    delete word, buttonContainer, seenButton, newButton;
    
    requestAnimationFrame(verbalMem);
}

requestAnimationFrame(verbalMem);

})();