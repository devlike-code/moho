#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"

UCLASS()
class ATestActor : public AActor
{
	GENERATED_BODY()
	
public:	
	// Sets default values for this actor's properties
	ATestActor();

protected:
	// Called when the game starts or when spawned
	virtual void BeginPlay() override;
private:	
	// Called every frame
	virtual void Tick(float DeltaTime) override;

protected:
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category="Object References", Description="A nice long description goes here")
    AAnotherActor m_AnotherActor;



public:
	AAnotherActor& GetAnotherActor() const { return m_AnotherActor; }
	const AAnotherActor& GetAnotherActor() const { return m_AnotherActor; }
	void SetAnotherActor(AAnotherActor& value) { m_AnotherActor = value; }


    UFUNCTION(BlueprintPure, Category="Game", meta=(WorldContext="WorldContextObject", UnsafeDuringActorConstruction="true"))
    APawn* GetPlayerPawn(const UObject* WorldContextObject, int32 PlayerIndex);


};