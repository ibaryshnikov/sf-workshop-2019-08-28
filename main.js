import init, { SceneContainer } from './pkg/shooter.js';

window.addEventListener('load', async () => {
    await init();
    const container = new SceneContainer();

    function draw() {
        requestAnimationFrame(draw);
        container.updateState();
        container.draw();
    }
    draw();
});
