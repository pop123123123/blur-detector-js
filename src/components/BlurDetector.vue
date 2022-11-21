<template>
  <div id="app">
    <input id="upload-photo" type="file" accept=".jpg,.jpeg,.heic,.png,application/pdf" name="upload document"
      max-size="20000000000" @change="filechange($event)" />
    <canvas id="imageCanvas" ref="imageCanvas"></canvas>
  </div>
</template>

<script>
import init, { variance_of_laplacian } from "../../blur/pkg/blur";

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
      this.clock();
      const blurIndex = await this.getBlurIndex(canvas);
      this.clock();
      const isBlur = blurIndex < this.threshold;
      console.log(blurIndex, isBlur);
      return isBlur;
    },
    async getBlurIndex(canvas) {
      const ctx = canvas.getContext('2d');
      const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
      // return await greet(imageData.data, canvas.width, canvas.height);
      // await greet(imageData.data, canvas.width, canvas.height);
      // console.log(imageData);
      return await variance_of_laplacian(imageData.data, canvas.width, canvas.height);
      // const outputImg = ctx.getImageData(0, 0, canvas.width, canvas.height);
      // const laplacian = (img, outputImgData) => {
      //   const kw = 3;
      //   const half = kw >> 1;
      //   const kernelIndices = Array(kw * kw).fill(0).map((_, i) => [(i % kw) - half, Math.floor(i / kw) - half]);

      //   const kernel = [0, 1, 0, 1, -4, 1, 0, 1, 0];

      //   for (let i = half; i < img.width - half; i++) {
      //     for (let j = half; j < img.height - half; j++) {
      //       let m = kernelIndices.map(([xk, yk], n) => img.data[(i + xk + (j + yk) * img.width) * 4] * kernel[n]).reduce((o, x) => o + x, 0);
      //       const a = (i + j * img.width) * 4;
      //       outputImgData[a] = m;
      //       outputImgData[a + 1] = m;
      //       outputImgData[a + 2] = m;
      //     }
      //   }

      //   // remove top and bottom artifacts
      //   for (let i = 0; i < img.width; i++) {
      //     for (let [aj, bj] of [[0, 1], [img.height - 1, img.height - 2]]) {
      //       const a = (i + aj * img.width) * 4;
      //       const b = (i + bj * img.width) * 4;
      //       outputImgData[a] = outputImgData[b];
      //       outputImgData[a + 1] = outputImgData[b + 1];
      //       outputImgData[a + 2] = outputImgData[b + 2];
      //     }
      //   }
      //   // remove left and right artifacts
      //   for (let i = 0; i < img.height; i++) {
      //     for (let [aj, bj] of [[0, 1], [img.width - 1, img.width - 2]]) {
      //       const a = (aj + i * img.width) * 4;
      //       const b = (bj + i * img.width) * 4;
      //       outputImgData[a] = outputImgData[b];
      //       outputImgData[a + 1] = outputImgData[b + 1];
      //       outputImgData[a + 2] = outputImgData[b + 2];
      //     }
      //   }
      // }
      // const sum = (arr) => arr.reduce((acc, x) => acc + x, 0);
      // const variance = (img) => {
      //   // map to array and "convert" to grayscale
      //   const data = Array(img.data.length / 4).fill(0).map((_, i) => img.data[i << 2]);
      //   const average = sum(data) / data.length;
      //   const variance = sum(data.map(x => Math.pow(x - average, 2))) / (data.length);
      //   return variance;
      // }
      // // laplacian(imageData, outputImg.data);
      // await laplacian(imageData.data, outputImg.data, canvas.width, canvas.height);
      // const index = variance(outputImg);
      // ctx.putImageData(outputImg, 0, 0);
      // this.displayCanvas(canvas, index);
      // return index;

    },
    // displayCanvas(canvas, value) {
    //   if (value) {
    //     const context = canvas.getContext("2d");
    //     context.fillStyle = "red";
    //     context.font = "bold 16px Arial";
    //     context.fillText(value, 50, 50);
    //   }
    //   document.getElementsByTagName('body')[0].appendChild(canvas);
    // },
    // FOR PROFILING PURPOSES
    clock() {
      if (window.clock)
      console.log(new Date().getTime() - window.clock.getTime());
      window.clock = new Date();
    }
  }
}
</script>
