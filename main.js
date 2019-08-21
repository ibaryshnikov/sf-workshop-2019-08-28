import init, { answer } from './pkg/shooter.js';

window.addEventListener('load', async () => {
    await init();
    const result = answer();
    console.log('the answer is', answer());
});
