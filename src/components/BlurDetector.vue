<template>
  <div id="app">
    <input id="upload-photo" type="file" accept=".jpg,.jpeg,.heic,.png,application/pdf" name="upload document"
      max-size="20000000000" @change="filechange($event)" />
    <canvas id="imageCanvas" ref="imageCanvas"></canvas>
  </div>
</template>

<script>
import init, { variance_of_laplacian } from "../../rust_blur_lib/blur";

export default {
  name: 'BlurDetector',
  props: {
    threshold: {
      type: Number,
      default: 130,
    },
  },
  components: {
  },
  methods: {
    filechange(event) {
      const input = event.target;
      if (input.files && input.files[0]) {
        const reader = new FileReader();
        reader.onloadend = async (e) => {
          if (e.target.result.indexOf('data:application/pdf') !== 0) {
            this.imgType = "image";
            const blur = await this.test(e.target.result);
            // result here
            this.$emit('result', blur);
          }
        }
        reader.readAsDataURL(input.files[0])
      }

    },
    async test(fileReaderResult) {
      const canvas = document.createElement('canvas');
      const img = new Image();
      img.src = fileReaderResult;
      await new Promise(function (resolve) {
        img.onload = resolve;
      });
      const ctx = canvas.getContext('2d');

      let w = img.width, h = img.height;
      canvas.height = h;
      canvas.width = w;
      ctx.drawImage(img, 0, 0, w, h);

      await init();
      const imageData = ctx.getImageData(0, 0, w, h);
      const blurIndex = await variance_of_laplacian(imageData.data, w, h);
      return blurIndex < this.threshold;
    },
  }
}
</script>
