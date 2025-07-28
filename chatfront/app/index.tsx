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

export default function Index() {
  return (
    <View className="flex-1 flex-col items-center justify-center w-full min-h-screen p-4 bg-white dark:bg-black">
      <Heading className="mt-4 mb-8 text-center text-2xl md:text-4xl">SSE Demo page</Heading>
      <View className="w-full max-w-2xl flex flex-col gap-6 items-center">
        <WebPost />
        <Webs />
<<<<<<< HEAD
            <Button size="md" variant="outline" action="primary">
      <ButtonText>Hello World!</ButtonText>

      
    </Button>
=======
  
>>>>>>> 08148ee2872eb3237b3146e941a7c825b08f41f3
      </View>
    </View>
  );
}