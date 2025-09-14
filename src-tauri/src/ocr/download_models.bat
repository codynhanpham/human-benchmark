@echo off
setlocal

set "DETECTION_MODEL=https://ocrs-models.s3-accelerate.amazonaws.com/text-detection.rten"
set "RECOGNITION_MODEL=https://ocrs-models.s3-accelerate.amazonaws.com/text-recognition.rten"

curl "%DETECTION_MODEL%" -o text-detection.rten
curl "%RECOGNITION_MODEL%" -o text-recognition.rten


