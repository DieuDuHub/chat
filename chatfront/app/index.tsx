import { Text, View } from "react-native";
import Webs from "./components/webs";
import WebPost from "./components/webpost";

//A ARCHIMATE : Web Site React-Native for Chat purpose using SSE

export default function Index() {
  return (
    <View
      style={{
        flex: 1,
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Text>SSE Demo page</Text>
      <View style={{ flexDirection: "row", justifyContent: "space-between" }}>
        <WebPost />
        <Webs />
      </View>
    </View>
  );
}
