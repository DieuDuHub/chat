import { View } from "react-native";
import React, { useState } from "react";
import { Button, ButtonText } from "@/components/ui/button";
import { Text } from "@/components/ui/text";
import { Input, InputField } from "@/components/ui/input"

export default function WebPost() {
  const [inputMessage, setInputMessage] = useState<string>("");
  const [responseMessage, setResponseMessage] = useState<string>("");

  const sendMessage = async () => {
    try {
      const response = await fetch("/api/message", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          room: "default",
          username: "user",
          message: inputMessage,
        }),
      });

      if (response.ok) {
        setResponseMessage("Message sent successfully!");
      } else {
        setResponseMessage("Failed to send message.");
      }
    } catch (error) {
      console.error("Error sending message:", error);
      setResponseMessage("Error sending message.");
    }
  };

  return (
    <View className="p-4 w-full max-w-md bg-white dark:bg-gray-900 rounded-lg shadow-md flex flex-col gap-4 items-center">
      <Text className="text-lg font-semibold mb-2 text-center">Send a Message</Text>
        <Input
      variant="outline"
      size="md"
      isDisabled={false}
      isInvalid={false}
      isReadOnly={false}
    >
        <InputField
          className="h-10  border-gray-300 dark:border-gray-700 rounded px-2 mb-2 bg-white dark:bg-gray-800 text-black dark:text-white focus:border-blue-500 focus:ring-2 focus:ring-blue-200"
          placeholder="Enter your message"
          value={inputMessage}
          onChangeText={setInputMessage}
          onSubmitEditing={sendMessage}
        />
        
      <Button size="md" variant="outline" action="primary" onPress={sendMessage}>
        <ButtonText>Send</ButtonText>
      </Button>
      </Input> 
      {responseMessage ? (
        <Text className="mt-2 text-blue-600 dark:text-blue-400 text-center">{responseMessage}</Text>
      ) : null}
    </View>
  );
}
// Input  w-full
// Button className="w-full md:w-auto mt-2" 