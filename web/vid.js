navigator.mediaDevices
.getUserMedia({ video: true })
.then((stream) => {
    const video = document.getElementById("video");
    video.srcObject = stream;
})
.catch((err) => {
    console.error("Error accessing camera:", err);
});