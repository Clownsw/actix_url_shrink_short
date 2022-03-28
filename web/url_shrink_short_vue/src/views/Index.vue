<template>
  <el-container>
    <el-main>
      <el-row :gutter="20">
        <el-col :span="6">
          <div class="grid-content bg-purple"/>
        </el-col>

        <el-col :span="12">
          <div class="grid-content bg-purple">
            <img class="logo" src="@/assets/logo.png" alt="">

            <div class="input-box">
              <el-input v-model="video_url" class="input-video-url" placeholder="请输入长链接地址"/>
              <el-button class="button-get-short-url" type="primary" @click="getShrinkShortUrl">获取短连接</el-button>
            </div>
          </div>
        </el-col>

        <el-col :span="6">
          <div class="grid-content bg-purple"/>
        </el-col>
      </el-row>
    </el-main>
  </el-container>
</template>

<script>
export default {
  name: "Index",
  data() {
    return {
      video_url: '',
    }
  },
  methods: {
    getShrinkShortUrl() {
      console.log(this.video_url)
    }
  }
}
</script>

<style scoped>
.input-box {
  display: flex;
}

.logo {
  display: block;
  width: 200px;
  height: 200px;
  margin: 35px auto;
}

.input-video-url ::v-deep .el-input__inner {
  height: 100%;
  font-size: 16px;
}

.select-video-type ::v-deep .select-trigger,
.select-video-type ::v-deep .select-trigger .el-input,
.select-video-type ::v-deep .select-trigger .el-input .el-input__inner {
  height: 100%;
}

.input-video-url ::v-deep .el-input__inner,
.select-video-type ::v-deep .select-trigger .el-input .el-input__inner {
  margin-right: 4px;
}

.button-get-short-url {
  height: 44px;
}
</style>