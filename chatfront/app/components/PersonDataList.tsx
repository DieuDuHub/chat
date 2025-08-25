import React, { useState, useEffect } from 'react';
import { ScrollView, RefreshControl, Alert } from 'react-native';
import { VStack } from "@/components/ui/vstack";
import { HStack } from "@/components/ui/hstack";
import { Box } from "@/components/ui/box";
import { Text } from "@/components/ui/text";
import { Button, ButtonText } from "@/components/ui/button";
import { Heading } from "@/components/ui/heading";
import { Badge, BadgeText } from "@/components/ui/badge";

// Type definitions based on PGBO API
interface PersonData {
  id: number;
  first_name: string;
  last_name: string;
  email: string;
  phone?: string;
  birth_date?: string;
  gender?: string;
  street_address?: string;
  city?: string;
  state_province?: string;
  postal_code?: string;
  country?: string;
  nationality?: string;
  occupation?: string;
  company?: string;
  salary?: number;
  marital_status?: string;
  created_at?: string;
  updated_at?: string;
  is_active?: boolean;
}

interface ApiResponse<T> {
  Ok?: T;
  Err?: string;
}

interface PersonDataListProps {
  apiBaseUrl?: string;
}

export default function PersonDataList({ 
  apiBaseUrl = 'http://localhost/referential' 
}: PersonDataListProps) {
  const [persons, setPersons] = useState<PersonData[]>([]);
  const [loading, setLoading] = useState(true);
  const [refreshing, setRefreshing] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchPersons = async () => {
    try {
      setError(null);
      const response = await fetch(`${apiBaseUrl}/person_data`);
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      
      const data: ApiResponse<PersonData[]> = await response.json();
      
      if (data.Ok) {
        setPersons(data.Ok);
      } else if (data.Err) {
        throw new Error(data.Err);
      } else {
        throw new Error('Invalid response format');
      }
    } catch (err) {
      console.error('Error fetching persons:', err);
      setError(err instanceof Error ? err.message : 'Unknown error occurred');
    } finally {
      setLoading(false);
      setRefreshing(false);
    }
  };

  useEffect(() => {
    fetchPersons();
  }, []);

  const onRefresh = () => {
    setRefreshing(true);
    fetchPersons();
  };

  const formatDate = (dateString?: string) => {
    if (!dateString) return 'N/A';
    return new Date(dateString).toLocaleDateString();
  };

  const formatSalary = (salary?: number) => {
    if (!salary) return 'N/A';
    return new Intl.NumberFormat('fr-FR', {
      style: 'currency',
      currency: 'EUR'
    }).format(salary);
  };

  if (loading) {
    return (
      <Box className="flex-1 justify-center items-center p-4">
        <Text>Chargement des données...</Text>
      </Box>
    );
  }

  if (error) {
    return (
      <Box className="flex-1 justify-center items-center p-4">
        <VStack space="md" className="items-center">
          <Text className="text-red-500 text-center">Erreur: {error}</Text>
          <Button onPress={fetchPersons}>
            <ButtonText>Réessayer</ButtonText>
          </Button>
        </VStack>
      </Box>
    );
  }

  return (
    <Box className="flex-1">
      <VStack space="lg" className="p-4">
        <HStack className="justify-between items-center">
          <Heading size="lg">Données Personnelles</Heading>
          <Button size="sm" onPress={fetchPersons}>
            <ButtonText>Actualiser</ButtonText>
          </Button>
        </HStack>

        <ScrollView
          refreshControl={
            <RefreshControl refreshing={refreshing} onRefresh={onRefresh} />
          }
          showsVerticalScrollIndicator={false}
        >
          <VStack space="md">
            {persons.length === 0 ? (
              <Box className="p-4 items-center">
                <Text>Aucune donnée disponible</Text>
              </Box>
            ) : (
              persons.map((person) => (
                <Box key={person.id} className="bg-white p-4 rounded-lg shadow-sm border border-gray-200">
                  <VStack space="sm">
                    <HStack className="justify-between items-start">
                      <VStack space="xs" className="flex-1">
                        <Text className="font-bold text-lg">
                          {person.first_name} {person.last_name}
                        </Text>
                        <Text className="text-blue-600">{person.email}</Text>
                      </VStack>
                      <Badge variant={person.is_active ? "success" : "secondary"}>
                        <BadgeText>{person.is_active ? "Actif" : "Inactif"}</BadgeText>
                      </Badge>
                    </HStack>

                    {person.phone && (
                      <Text className="text-gray-600">📞 {person.phone}</Text>
                    )}

                    {person.birth_date && (
                      <Text className="text-gray-600">
                        🎂 Né(e) le {formatDate(person.birth_date)}
                      </Text>
                    )}

                    {person.occupation && (
                      <Text className="text-gray-600">
                        💼 {person.occupation}
                        {person.company && ` chez ${person.company}`}
                      </Text>
                    )}

                    {person.salary && (
                      <Text className="text-green-600 font-semibold">
                        💰 {formatSalary(person.salary)}
                      </Text>
                    )}

                    {(person.street_address || person.city) && (
                      <Text className="text-gray-600">
                        📍 {[person.street_address, person.city, person.country]
                          .filter(Boolean)
                          .join(', ')}
                      </Text>
                    )}

                    <Text className="text-xs text-gray-400 mt-2">
                      ID: {person.id} • Créé le {formatDate(person.created_at)}
                    </Text>
                  </VStack>
                </Box>
              ))
            )}
          </VStack>
        </ScrollView>
      </VStack>
    </Box>
  );
}
