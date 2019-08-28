import init, { answer } from './pkg/shooter.js';

window.addEventListener('load', async () => {
    await init();
    console.log('the answer is', answer());
});
