import QtQuick
import QtQuick.Controls

ApplicationWindow {
    id: root  // 👈 必须加上这个，否则下文的 root.height 会失效
    visible: true
    width: 800
    height: 800
    title: "SuperSpaceNeo-Dev Mode"

    function debugMode() { // 建议小写开头，符合 JS 函数习惯
        console.log("Developing - Current Scale: " + (root.height * 0.1))
    }

    // 使用 Column 容器让它们垂直排列，别叠在一起
    Column {
        anchors.centerIn: parent
        spacing: 30

        Text {
            text: "SuperSpaceNeo Dev Mode"
            // 选 A 是对的！使用 Math.round 或 Math.max 增加鲁棒性
            font.pixelSize: Math.max(12, root.height * 0.05)
            anchors.horizontalCenter: parent
        }

        Button {
            text: "调试"
            anchors.horizontalCenter: parent
            onClicked: {
                root.debugMode()
            }
        }
    }
}