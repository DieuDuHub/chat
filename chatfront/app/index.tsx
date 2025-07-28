import { Text, View } from "react-native";
import {
  DarkTheme,
  DefaultTheme,
  ThemeProvider,
} from "@react-navigation/native";
import Webs from "./components/webs";
import WebPost from "./components/webpost";
import { Heading } from "../components/ui/heading"
import { Button, ButtonText } from "@/components/ui/button"
//import { GluestackUIProvider } from "@/components/ui/gluestack-ui-provider";


//A ARCHIMATE : Web Site React-Native for Chat purpose using SSE
import "../global.css";

export default  function Index() {
  return (
    <View
      style={{
        flex: 1,
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Heading className="mt: 4">SSE Demo page</Heading>
      <View style={{ flexDirection: "row", justifyContent: "space-between" }}>
        <WebPost />
        <Webs />
            <Button size="md" variant="outline" action="primary">
      <ButtonText>Hello World!</ButtonText>

      
    </Button>
      </View>
    </View>
  );
}