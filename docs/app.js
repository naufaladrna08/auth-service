const express = require('express');
const swaggerUi = require('swagger-ui-express');
const fs = require('fs');
const path = require('path');

const app = express();
const PORT = 3000;

// Serve Swagger UI
app.use('/docs', swaggerUi.serve);

app.get('/docs/:service', (req, res) => {
  const { service } = req.params;
  const swaggerFilePath = path.join(__dirname, 'specs', `${service}.swagger.json`);

  // Check if the Swagger JSON file exists
  if (fs.existsSync(swaggerFilePath)) {
    swaggerUi.setup(require(swaggerFilePath))(req, res);
  } else {
    res.status(404).send({ error: `Swagger documentation for '${service}' not found` });
  }
});

app.listen(PORT, () => {
  console.log(`Server is running on http://localhost:${PORT}`);
});
