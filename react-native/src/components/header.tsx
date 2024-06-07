import { Text, View } from "react-native";

interface HeaderProps {
  title: string;
}

export function Header({ title }: HeaderProps) {
  return (
    <View className="h-28 w-full flex-row items-end border-b border-white/10 bg-black/20 px-8 pb-4">
      <Text className="flex-1 text-center font-medium text-lg text-white">
        {title}
      </Text>
    </View>
  );
}
