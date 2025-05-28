import { Box } from "@/components/ui/box";
import {
  Button,
  ButtonGroup,
  ButtonIcon,
  ButtonSpinner,
  ButtonText,
} from "@/components/ui/button";
import { Heading } from "@/components/ui/heading";
import { Text } from "@/components/ui/text";
import { View, } from "react-native";

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
      <Box className="bg-primary-500 p-5">This is a box</Box>
      <Text className="bg-primary-500 p-5">This is a box</Text>
      <Heading className="text-typography-950 font-semibold" size="md">This is a d</Heading>
      <ButtonGroup>
          <Button>
            <ButtonText>Hey</ButtonText>
            <ButtonSpinner />
            <ButtonIcon />
          </Button>
        </ButtonGroup>
        <input
        className="bg-white border border-gray-300 rounded-lg py-2 px-4 w-full leading-normal focus:outline-none focus:ring-2 focus:ring-primary-500 transition-shadow"
        type="email"
        placeholder="jane@example.com"
      />
    </View>
  );
}
