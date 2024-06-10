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
import { useBadgeStore } from "@/store/badge-store";
import { Redirect } from "expo-router";
import { Share } from "react-native";
import { MotiView } from "moti";

export default function Ticket() {
  const [isQRCodeExpanded, setIsQRCodeExpanded] = useState(false);

  const badgeStore = useBadgeStore();

  async function handleShare() {
    if (badgeStore.data?.checkInUrl) {
      await Share.share({
        message: badgeStore.data.checkInUrl,
      });
    }
    try {
    } catch (error) {
      Alert.alert(
        "Compartilhar",
        "Não foi possível compartilhar a credencial.",
      );
    }
  }

  async function handleSelectImage() {
    try {
      const result = await ImagePicker.launchImageLibraryAsync({
        mediaTypes: ImagePicker.MediaTypeOptions.Images,
        allowsEditing: true,
        aspect: [4, 4],
      });

      if (result.assets) {
        badgeStore.updateAvatar(result.assets[0].uri);
      }
    } catch (error) {
      Alert.alert("Foto", "Não foi possível selecionar a imagem.");
    }
  }

  if (!badgeStore.data?.checkInUrl) {
    return <Redirect href="/" />;
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
          data={badgeStore.data}
          onChangeAvatar={handleSelectImage}
          onExpandQRCode={() => setIsQRCodeExpanded(true)}
        />

        <MotiView
          from={{
            translateY: 0,
          }}
          animate={{
            translateY: 10,
          }}
          transition={{
            loop: true,
            type: "timing",
            duration: 700,
          }}
        >
          <FontAwesome
            name="angle-double-down"
            size={24}
            color={colors.gray[300]}
            className="my-6 self-center"
          />
        </MotiView>

        <Text className="mt-4 font-bold text-2xl text-white">
          Compartilhar credencial
        </Text>

        <Text className="mb-6 mt-1 font-regular text-base text-white">
          Mostre ao mundo que você vai participar do{" "}
          {badgeStore.data.eventTitle}!
        </Text>

        <Button title="Compartilhar" onPress={handleShare} />

        <TouchableOpacity
          activeOpacity={0.7}
          className="mt-10"
          onPress={badgeStore.remove}
        >
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
