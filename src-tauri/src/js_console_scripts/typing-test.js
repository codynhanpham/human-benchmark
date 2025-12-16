(async () => {
    const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
    const range = (start, stop, step = 1) =>
        Array.from(
            { length: (stop - start) / step + 1 },
            (_, i) => start + i * step
    );
    function inputFieldExists() {
        return document.querySelector('.letters.notranslate') !== null;
    }
    async function typeTest(slowmedown=0) {
        const inputField = document.querySelector('.letters.notranslate');
        if (!inputField) {
            console.warn("Input field not found. This can happen if the test is not loaded yet. Simply re-run the script.");
            return;
        }
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
                // // Use requestAnimationFrame to guarantee the result is registered, but a bit slower (1M-2M WPM)
                // requestAnimationFrame(() => {
                //     typeCharacter(testContent[testContent.length - 1]);
                // });

                // // Type the last character immediately, will need some retries, ideally run on tests with really long text
                // Will reaches > 6M WPM, but HumanBenchmark may not register the result and report 0 WPM

                // To automate this: run typeTest in a loop and gradually increase slowmedown (just do some random thing to eat up some cpu cycles) until get non-zero WPM
                for (let j = 0; j < slowmedown; j++) {
                    Math.sin(Math.random()) // pick some slow operation
                }
                typeCharacter(testContent[testContent.length - 1]);
            }
        }
    }
    function nonZeroWPM() {
        const resultDiv = document.querySelector('div[data-test="true"] h1');
        const wpm = parseInt(resultDiv.innerText);
        return wpm > 0;
    }
    function restart() {
        const restartButton = document.querySelector('div[data-test="true"] button.secondary');
        if (restartButton) {
            restartButton.click();
        }
    }
    function skipFrame() {
        return new Promise((resolve) => requestAnimationFrame(resolve));
    }

    const ATTEMPTS = range(15, 312, 3); // Try different slowmedown values, you can optimize this, but this is typically a good default
    let attemptIndex = 0;
    restart();
    while (attemptIndex < ATTEMPTS.length) {
        // If the test isn't loaded yet, wait and retry without consuming an attempt.
        while (!inputFieldExists()) {
            await sleep(100);
        }

        const slowmedown = ATTEMPTS[attemptIndex];
        console.log(`Attempting typing test with slowmedown = ${slowmedown} (attempt ${attemptIndex + 1}/${ATTEMPTS.length})`);
        // Wait a frame to ensure the input field is ready
        await skipFrame();
        await typeTest(slowmedown);
        // Real fast if simply wait for next frame, if this fails, you can try to add some delay after this
        await skipFrame();
        // await sleep(50);

        if (nonZeroWPM()) {
            const wpmText = document.querySelector('div[data-test="true"] h1').innerText;
            const wpm = Number.parseInt(wpmText, 10);
            const wpmFormatted = wpm.toLocaleString('en-US').replaceAll(',', '_');

            console.log(`Typing test completed successfully with non-zero WPM on attempt ${attemptIndex + 1}:\n ${wpmFormatted} WPM`);
            break;
        }
        await skipFrame();
        restart();
        attemptIndex++;
    }
})();