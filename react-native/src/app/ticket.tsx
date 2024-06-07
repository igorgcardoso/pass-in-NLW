import {
  Alert,
  Modal,
  ScrollView,
  Text,
  TouchableOpacity,
  View,
} from "react-native";
import { Header } from "@/components/header";
import { Credential } from "@/components/credential";
import { FontAwesome } from "@expo/vector-icons";
import { colors } from "@/styles/colors";
import { Button } from "@/components/button";
import { useState } from "react";
import * as ImagePicker from "expo-image-picker";
import { QRCode } from "@/components/qrcode";

export default function Ticket() {
  const [image, setImage] = useState("");
  const [isQRCodeExpanded, setIsQRCodeExpanded] = useState(false);

  async function handleSelectImage() {
    try {
      const result = await ImagePicker.launchImageLibraryAsync({
        mediaTypes: ImagePicker.MediaTypeOptions.Images,
        allowsEditing: true,
        aspect: [4, 4],
      });

      if (result.assets) {
        setImage(result.assets[0].uri);
      }
    } catch (error) {
      Alert.alert("Foto", "Não foi possível selecionar a imagem.");
    }
  }

  return (
    <View className="flex-1 bg-green-500">
      <Header title="Minha credential" />

      <ScrollView
        className="-z-10 -mt-28"
        contentContainerClassName="px-8 pb-8"
        showsVerticalScrollIndicator={false}
      >
        <Credential
          onChangeAvatar={handleSelectImage}
          image={image}
          onExpandQRCode={() => setIsQRCodeExpanded(true)}
        />

        <FontAwesome
          name="angle-double-down"
          size={24}
          color={colors.gray[300]}
          className="my-6 self-center"
        />

        <Text className="mt-4 font-bold text-2xl text-white">
          Compartilhar credencial
        </Text>

        <Text className="mb-6 mt-1 font-regular text-base text-white">
          Mostre ao mundo que você vai participar do Unite Summit!
        </Text>

        <Button title="Compartilhar" />

        <TouchableOpacity activeOpacity={0.7} className="mt-10">
          <Text className="text-center font-bold text-base text-white">
            Remover Ingresso
          </Text>
        </TouchableOpacity>
      </ScrollView>

      <Modal visible={isQRCodeExpanded} statusBarTranslucent>
        <View className="flex-1 items-center justify-center bg-green-500">
          <QRCode value="test" size={300} />
          <TouchableOpacity
            activeOpacity={0.7}
            className="mt-10"
            onPress={() => setIsQRCodeExpanded(false)}
          >
            <Text className="font-body text-sm text-orange-500">
              Fechar QRCode
            </Text>
          </TouchableOpacity>
        </View>
      </Modal>
    </View>
  );
}
