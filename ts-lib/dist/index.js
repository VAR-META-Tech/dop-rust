import { app } from './api.js';
app.listen(3000, () => {
    console.log('TS Engine API running...');
});
app.get('/health', (req, res) => {
    res.send('OK');
});
