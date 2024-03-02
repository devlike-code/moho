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

private:
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category="Object References", Description="A nice long description goes here")
    TObjectPtr<AAnotherActor> m_AnotherActor;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category="Object References", Description="Ugly af")
    TObjectPtr<AYetAnotherActor> m_YetAnother;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category="Settings", ClampMin=10, ClampMax=15)
    float m_MaxHealth = 10;

    UPROPERTY(BlueprintReadOnly, Transient, Description="Read only var")
    bool m_IsTrue = false;



public:
	TObjectPtr<AAnotherActor>& GetAnotherActor() const { return m_AnotherActor; }
	const TObjectPtr<AAnotherActor>& GetAnotherActor() const { return m_AnotherActor; }
	void SetAnotherActor(TObjectPtr<AAnotherActor>& value) { m_AnotherActor = value; }

	TObjectPtr<AYetAnotherActor>& GetYetAnother() const { return m_YetAnother; }
	const TObjectPtr<AYetAnotherActor>& GetYetAnother() const { return m_YetAnother; }
	void SetYetAnother(TObjectPtr<AYetAnotherActor>& value) { m_YetAnother = value; }

	float& GetMaxHealth() const { return m_MaxHealth; }
	const float& GetMaxHealth() const { return m_MaxHealth; }
	void SetMaxHealth(float& value) { m_MaxHealth = value; }

	bool& GetIsTrue() const { return m_IsTrue; }
	const bool& GetIsTrue() const { return m_IsTrue; }
	void SetIsTrue(bool& value) { m_IsTrue = value; }


};