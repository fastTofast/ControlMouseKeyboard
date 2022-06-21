<script setup lang="ts">
import { ref, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import Draggable from 'vuedraggable'
// defineProps<{ msg: string }>()
enum OpType {
  CLICKRIGHT = 'CLICKRIGHT',
  CLICKLEFT = 'CLICKLEFT',
  MOVE = 'MOVE',
  MOVERELATE = 'MOVERELATE',
  KEYDOWN = 'KEYDOWN',
  KEYCLICK = 'KEYCLICK',
  KEYUP = 'KEYUP',
  KEYINPUT = 'KEYINPUT',
  DELAY = 'DELAY',
}
const OpTypeName = {
  CLICKRIGHT: "点击右键",
  CLICKLEFT: "点击左键",
  MOVE: "移动（绝对）",
  MOVERELATE: "移动（相对）",
  KEYDOWN: "按下键盘按键",
  KEYCLICK: "点击键盘按键",
  KEYUP: "松开键盘按键",
  KEYINPUT: "字符输入",
  DELAY: "延时"
}
interface OpItem {
  type: OpType,
  count: Number,
  position?: { x: Number, y: Number },
  word?: String
  time?: Number
}
const form = ref({
  x: 0,
  y: 0,
  count: 1,
})
function move(e: Event) {
  invoke('move_mouse', { posStr: JSON.stringify(form.value) })
  console.log('----------', form.value.x)
}
console.log('OpType: ', OpType.CLICKLEFT);
const list = ref<Array<OpItem>>([
  { type: OpType.CLICKRIGHT, count: 1, },
  { type: OpType.CLICKLEFT, count: 1, },
  { type: OpType.KEYCLICK, count: 1, word: 'a' },
  { type: OpType.MOVE, count: 1, position: { x: 0, y: 0 } },
  { type: OpType.MOVERELATE, count: 1, position: { x: 0, y: 0 } },
  { type: OpType.MOVERELATE, count: 1, position: { x: 0, y: 0 } },
])
let addedType = ref(OpType.CLICKRIGHT);
function addType(e: Event) {
  let config: OpItem = {
    type: addedType.value,
    count: 0,
  }
  if (addedType.value === OpType.DELAY) {
    config.time = 0
  } else if (addedType.value === OpType.MOVE || addedType.value === OpType.MOVERELATE) {
    config.position = {
      x: 0, y: 0
    }
  } else if (addedType.value === OpType.KEYCLICK || addedType.value === OpType.KEYUP || addedType.value === OpType.KEYDOWN || addedType.value === OpType.KEYINPUT) {
    config.word = ''
    config.count = 1
  }
  list.value.push(config)
}
</script>

<template>
  <div class="content">
    <div class="header">
      <select v-model="addedType" @change.lazy="addType">
        <option :value="op" v-for="op in OpType">{{ OpTypeName[op as never] }}</option>
      </select>
      <span @click="move" class="btn">执行</span>
    </div>
    <div class="op-content">
      <Draggable v-model="list" :animation="300" handle=".op-name">
        <template #item="{ element: opItem }">
          <div>
            <div class="item" v-if="opItem.type === OpType.MOVE || opItem.type === OpType.MOVERELATE">
              <div class="op-name"><span>{{ OpTypeName[opItem.type as never] }}</span></div>
              <div class="op-section">
                <span>X坐标:</span>
                <input v-model.number="opItem.position.x" type="text" placeholder="X">
                <span>Y坐标:</span>
                <input v-model.number="opItem.position.y" type="text" placeholder="Y">
                <span>次数</span>
                <input v-model.number="opItem.count" type="text" placeholder="次数">
              </div>
            </div>
            <div class="item" v-else-if="opItem.type === OpType.CLICKLEFT || opItem.type === OpType.CLICKRIGHT">
              <div class="op-name"><span>{{ OpTypeName[opItem.type as never] }}</span></div>
              <div class="op-section">
                <span>次数</span>
                <input v-model.number="opItem.count" type="text" placeholder="次数">
              </div>
            </div>
            <div class="item"
              v-else-if="opItem.type === OpType.KEYDOWN || opItem.type === OpType.KEYCLICK || opItem.type === OpType.KEYUP || opItem.type === OpType.KEYINPUT">
              <div class="op-name"><span>{{ OpTypeName[opItem.type as never] }}</span></div>
              <div class="op-section">
                <span>字符</span>
                <input v-model.number="opItem.word" type="text" placeholder="字符">
                <span>次数</span>
                <input v-model.number="opItem.count" type="text" placeholder="次数">
              </div>
            </div>
            <div class="item" v-else-if="opItem.type === OpType.DELAY">
              <div class=" op-name"><span>{{ OpTypeName[opItem.type as never] }}</span></div>
              <div class="op-section">
                <span>毫秒数:</span>
                <input v-model.number="opItem.time" type="text" placeholder="毫秒数">
              </div>
            </div>
          </div>
        </template>
      </Draggable>
    </div>
  </div>
</template>

<style scoped lang="less">
.content {
  width: 80%;
  margin: auto;
  display: flex;
  justify-content: flex-start;
  align-items: start;
  flex-direction: column;

  .header {
    margin-bottom: 10px;
  }

  .op-content {
    flex: 1;
    width: 100%;

    .item {
      padding: 10px;
      border: 1px solid #686968;
      margin-bottom: 20px;
      background-color: #165a5a;
      border-radius: 4px;
      display: flex;
      color: #f9f9f9;
      user-select: none;

      .op-name {
        flex: 0 0 100px;
        display: flex;
        align-items: center;
        margin-right: 20px;
        line-height: 1;
        cursor: move;
      }

      .op-section {
        flex: 1 0 auto;
        display: flex;
        align-content: center;

        input {
          flex: 1;
          width: 30px;
          border-radius: 4px;
          outline: none;
          border: 1px solid #686968;
          height: 30px;
          padding: 0 3px;
          background-color: #a8c7ff;
          color: #000;
          font-size: 16px;
          font-weight: bold;
        }

        span {
          flex: 0 0 60px;
          margin-left: 20px;
          line-height: 30px;
        }
      }
    }
  }
}

.btn {
  color: #ffffff;
  border-radius: 4px;
  cursor: pointer;
  border: 1px solid #686968;
  padding: 4px;
  background-color: #070707;
}
</style>
