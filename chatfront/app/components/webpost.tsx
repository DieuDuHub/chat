import { View, Text, TextInput, Button } from "react-native";
import React, { useState } from "react";

export default function WebPost() {
  const [inputMessage, setInputMessage] = useState<string>("");
  const [responseMessage, setResponseMessage] = useState<string>("");

  const sendMessage = async () => {
    try {
      const response = await fetch("http://localhost/api/message", {
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
    <View style={{ padding: 20 }}>
      <Text style={{ fontSize: 18, marginBottom: 10 }}>Send a Message</Text>
      <TextInput
        style={{
          height: 40,
          borderColor: "gray",
          borderWidth: 1,
          marginBottom: 10,
          paddingHorizontal: 8,
        }}
        placeholder="Enter your message"
        value={inputMessage}
        onChangeText={setInputMessage}
        onSubmitEditing={sendMessage} // Trigger sendMessage on Return key
      />
      <Button title="Send" onPress={sendMessage} />
      {responseMessage ? (
        <Text style={{ marginTop: 10, color: "blue" }}>{responseMessage}</Text>
      ) : null}
    </View>
  );
}