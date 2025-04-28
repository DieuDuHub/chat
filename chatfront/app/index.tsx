import { Text, View } from "react-native";
import Webs from "./components/webs";

export default function Index() {
  return (
    <View
      style={{
        flex: 1,
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Text>Edit app/index.tsx to edit this screen.</Text>
      <Webs/>
    </View>
  );
}
