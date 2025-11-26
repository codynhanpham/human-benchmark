(() => {
    const inputField = document.querySelector('.letters.notranslate');
    const testContent = inputField.textContent;
    const typeCharacter = (char) => {
        const keydownEvent = new KeyboardEvent('keydown', { key: char, bubbles: true });
        const keyupEvent = new KeyboardEvent('keyup', { key: char, bubbles: true });

        inputField.dispatchEvent(keydownEvent);
        inputField.dispatchEvent(keyupEvent);
    };
    for (let i = 0; i < testContent.length-1; i++) {
        const char = testContent[i];
        typeCharacter(char);
        if (i === testContent.length - 2) {
            // Use requestAnimationFrame to guarantee the result is registered, but a bit slower (1M-2M WPM)
            // requestAnimationFrame(() => {
            //     typeCharacter(testContent[testContent.length - 1]);
            // });

            // Type the last character immediately, will need some retries, ideally run on tests with really long text
            // Will reaches > 6M WPM, but HumanBenchmark may not register the result and report 0 WPM
            typeCharacter(testContent[testContent.length - 1]);
        }
    }
})();