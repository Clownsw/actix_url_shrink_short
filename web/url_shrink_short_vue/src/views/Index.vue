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
              <el-input v-model="url_obj.url" class="input-video-url" placeholder="请输入长链接地址"/>
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
import {h} from 'vue'
import {ElMessage, ElMessageBox} from 'element-plus'

export default {
  name: "Index",
  data() {
    return {
      url_obj: {
        url: '',
      }
    }
  },
  methods: {
    getShrinkShortUrl() {
      if (this.url_obj.url === '') {
        this.errorMessageBox('请输入长链接!');
      } else {
        if (this.url_obj.url.indexOf('http://') < 0 && this.url_obj.url.indexOf('https://') < 0) {
          this.errorMessageBox('请输入正确的长链接!');
        } else {
          this.$axios.post('http://localhost:8888/api/add', JSON.stringify(this.url_obj))
              .then(resp => {
                console.log(resp)
                if (resp.data.code === 200) {
                  ElMessageBox({
                    title: 'Success!',
                    message: h('p', null, [
                      h('i', {style: 'color: teal'}, 'http://localhost:8888/api/t/' + resp.data.message),
                    ]),
                    showCancelButton: false,
                    confirmButtonText: 'OK',
                  })
                }
              })
              .catch(error => {
                console.log(error)
              })
        }
      }
    },
    errorMessageBox(message) {
      ElMessage.error(message)
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

/*.input-video-url ::v-deep .el-input__inner {*/
/*  height: 100%;*/
/*  font-size: 16px;*/
/*}*/

.input-video-url ::v-deep(.el-input__inner) {
  height: 100%;
  font-size: 16px;
}

/*.select-video-type ::v-deep .select-trigger,*/
/*.select-video-type ::v-deep .select-trigger .el-input,*/
/*.select-video-type ::v-deep .select-trigger .el-input .el-input__inner {*/
/*  height: 100%;*/
/*}*/

.select-video-type ::v-deep(.el-input__inner) {
  height: 100%;
}

/*.input-video-url ::v-deep .el-input__inner,*/
/*.select-video-type ::v-deep .select-trigger .el-input .el-input__inner {*/
/*  margin-right: 4px;*/
/*}*/

.input-video-url ::v-deep(.el-input__inner) {
  margin-right: 4px;
}

.button-get-short-url {
  height: 44px;
}
</style>