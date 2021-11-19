(async function() {
    let wasm = await import("../pkg/index_bg.wasm");
    let video_poker = await import("../pkg/index.js");

    const memoryView = new Uint8Array(wasm.memory.buffer);

    video_poker.setup();
    const displayAddr = video_poker.get_display();
    const displayWidth = video_poker.get_display_width();
    const displayHeight = video_poker.get_display_height();
    const displaySize = displayWidth * displayHeight;

    const gameCanvas = document.getElementById("game-canvas");
    document.addEventListener('keydown', e => {
        video_poker.key_down(e.keyCode)
    });
    document.addEventListener('keyup', e => {
        video_poker.key_up(e.keyCode)
    });
    gameCanvas.addEventListener('mousemove', e => {
        video_poker.mouse_move(e.offsetX, e.offsetY);
    });
    gameCanvas.addEventListener('mousedown', e => {
        video_poker.mouse_down(e.offsetX, e.offsetY);
    });
    gameCanvas.addEventListener('mouseup', e => {
        video_poker.mouse_up(e.offsetX, e.offsetY);
    });

    const ctx = gameCanvas.getContext('2d');

    let start;
    function step(timestamp) {
        if (start === undefined) {
            start = timestamp;
        }
        const dt = (timestamp - start) * 0.001;
        start = timestamp;

        const frame = new ImageData(
            new Uint8ClampedArray(
                memoryView.subarray(
                    displayAddr,
                    displayAddr + 4 * displaySize)),
            displayWidth, displayHeight);
        ctx.putImageData(frame, 0, 0);

        video_poker.update_suggestions();

        window.requestAnimationFrame(step);
    }
    window.requestAnimationFrame(step);
})();
