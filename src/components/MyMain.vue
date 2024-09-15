<template>
  <el-container>
    <el-header>Header</el-header>
    <el-container>
      <el-aside width="200px">Aside</el-aside>
      <el-main>
        Main
        <el-button type="text" @click="open">点击打开 Message Box</el-button>
        <el-button type="primary" @click="load">加载</el-button>
      </el-main>
    </el-container>
  </el-container>
</template>

<script>
const {invoke} = window.__TAURI__.tauri;
export default {
  methods: {
    open() {
      this.$prompt('请输入邮箱', '提示', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        inputErrorMessage: '邮箱格式不正确'
      }).then(({ value }) => {
        // this.$message({
        //   type: 'success',
        //   message: '你的邮箱是: ' + value
        // });
        invoke('first_tauri',{str: value})
            .then((rep)=>{
              this.$message({
                  type: 'success',
                  message: '你的邮箱是: ' + rep
                });
            })
      }).catch(() => {
        this.$message({
          type: 'info',
          message: '取消输入'
        });
      });
    },
    load(){
      invoke('load')
          .then((rep)=>{
            console.log(rep)
          })
    }
  }
}
</script>

<style scoped>
.el-header, .el-footer {
  background-color: #B3C0D1;
  color: #333;
  text-align: center;
  line-height: 60px;
}

.el-aside {
  background-color: #D3DCE6;
  color: #333;
  text-align: center;
  line-height: 200px;
}

.el-main {
  background-color: #E9EEF3;
  color: #333;
  text-align: center;
  line-height: 160px;
}

body > .el-container {
  margin-bottom: 40px;
}

.el-container:nth-child(5) .el-aside,
.el-container:nth-child(6) .el-aside {
  line-height: 260px;
}

.el-container:nth-child(7) .el-aside {
  line-height: 320px;
}
</style>