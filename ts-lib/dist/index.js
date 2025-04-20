import { app } from './api/index.js';
import { PORT } from './config.js';
app.listen(PORT, () => {
    console.log(`TS Engine API running on port ${PORT}`);
});
