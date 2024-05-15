async function init() {
  let rustApp = null;

  try {
    rustApp = await import("../pkg");
  } catch (error) {
    console.error(error);
    return;
  }

  const input = document.getElementById("upload");
  const img = document.getElementById("new-img");
  const fileReader = new FileReader();

  fileReader.onloadend = () => {
    const base64 = fileReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ""
    );

    const imgDataUrl = rustApp.grayscale(base64);

    img.setAttribute("src", imgDataUrl);
  };

  input.addEventListener("change", (event) => {
    fileReader.readAsDataURL(event.target.files[0]);
  });
}

init();
