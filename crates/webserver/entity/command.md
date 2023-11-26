`sea generate entity -lo ./src --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")'`

Run this command in the entities directory!
**This will overwrite existing files!**

## Important shit
- DateTime should be changed to DateTimeUtc
- Password fields should be given the Serde skip attribute

---
For small changes change the output directory and manually copy over the changes to the entities.