<?php

namespace App\Entity;

use App\Enum\Level;
use App\Repository\UserRepository;
use App\Entity\HikingGroup;
use Doctrine\Common\Collections\ArrayCollection;
use Doctrine\Common\Collections\Collection;
use Doctrine\ORM\Mapping as ORM;
use Symfony\Component\Security\Core\User\PasswordAuthenticatedUserInterface;
use Symfony\Component\Security\Core\User\UserInterface;

#[ORM\Entity(repositoryClass: UserRepository::class)]
#[ORM\UniqueConstraint(name: 'UNIQ_IDENTIFIER_EMAIL', fields: ['email'])]
#[ORM\UniqueConstraint(name: 'UNIQ_IDENTIFIER_USERNAME', fields: ['username'])]
class User implements UserInterface, PasswordAuthenticatedUserInterface
{
    #[ORM\Id]
    #[ORM\GeneratedValue]
    #[ORM\Column]
    private ?int $id = null;

    #[ORM\Column(length: 180)]
    private ?string $email = null;

    /**
     * @var list<string> The user roles
     */
    #[ORM\Column]
    private array $roles = [];

    /**
     * @var string The hashed password
     */
    #[ORM\Column]
    private ?string $password = null;


    #[ORM\Column(length: 50, unique: true, nullable: false)]
    private ?string $username = null;


    #[ORM\Column(length: 255, nullable: true)]
    private ?string $avatar = null;

    #[ORM\Column(type: 'text', nullable: true)]
    private ?string $bio = null;

    #[ORM\Column(length: 50, nullable: true)]
    private ?string $level = null;

    #[ORM\Column(length: 100, nullable: true)]
    private ?string $location = null;

    #[ORM\Column(type: 'datetime')]
    private ?\DateTimeInterface $joinDate = null;

    #[ORM\OneToMany(targetEntity: Contact::class, mappedBy: 'user', orphanRemoval: true)]
    private Collection $contacts;

    #[ORM\OneToMany(targetEntity: Friendship::class, mappedBy: 'sender', cascade: ['remove'])]
    private Collection $friendsSent;

    #[ORM\OneToMany(targetEntity: Friendship::class, mappedBy: 'receiver', cascade: ['remove'])]
    private Collection $friendsReceived;

    #[ORM\ManyToMany(targetEntity: HikingGroup::class, mappedBy: 'participants')]
    private Collection $groupsJoined;

    
    public function getGroupsJoined(): Collection
    {
        return $this->groupsJoined;
    }

    public function addGroupJoined(HikingGroup $group): static
    {
        if (!$this->groupsJoined->contains($group)) {
            $this->groupsJoined->add($group);
            $group->addParticipant($this);
        }
        return $this;
    }

    public function removeGroupJoined(HikingGroup $group): static
    {
        if ($this->groupsJoined->removeElement($group)) {
            $group->removeParticipant($this);
        }
        return $this;
    }


    public function __construct()
    {
        $this->contacts = new ArrayCollection();
        $this->joinDate = new \DateTime();
    }

    public function getId(): ?int
    {
        return $this->id;
    }

    public function getEmail(): ?string
    {
        return $this->email;
    }

    public function setEmail(string $email): static
    {
        $this->email = $email;
        return $this;
    }

    public function getUserIdentifier(): string
    {
        return $this->email;
    }

    public function getRoles(): array
    {
        $roles = $this->roles;
        $roles[] = 'ROLE_USER';
        return array_unique($roles);
    }

    public function setRoles(array $roles): static
    {
        $this->roles = $roles;
        return $this;
    }

    public function getPassword(): ?string
    {
        return $this->password;
    }

    public function setPassword(string $password): static
    {
        $this->password = $password;
        return $this;
    }

    public function eraseCredentials(): void
    {
        // If you store any temporary, sensitive data on the user, clear it here
    }

    // Contacts
    /**
     * @return Collection<int, Contact>
     */
    public function getContacts(): Collection
    {
        return $this->contacts;
    }

    public function addContact(Contact $contact): static
    {
        if (!$this->contacts->contains($contact)) {
            $this->contacts->add($contact);
            $contact->setUser($this);
        }
        return $this;
    }

    public function removeContact(Contact $contact): static
    {
        if ($this->contacts->removeElement($contact)) {
            if ($contact->getUser() === $this) {
                $contact->setUser(null);
            }
        }
        return $this;
    }

    
    public function getUsername(): ?string
    {
        return $this->username;
    }

    public function setUsername(string $username): static
    {
        $this->username = $username;
        return $this;
    }


    public function getAvatar(): ?string
    {
        return $this->avatar;
    }

    public function setAvatar(?string $avatar): static
    {
        $this->avatar = $avatar;
        return $this;
    }

    public function getBio(): ?string
    {
        return $this->bio;
    }

    public function setBio(?string $bio): static
    {
        $this->bio = $bio;
        return $this;
    }

    public function getLevel(): ?Level
    {
        return $this->level !== null ? Level::from($this->level) : null;
    }

    public function setLevel(?Level $level): static
    {
        $this->level = $level?->value;
        return $this;
    }

    public function getLocation(): ?string
    {
        return $this->location;
    }

    public function setLocation(?string $location): static
    {
        $this->location = $location;
        return $this;
    }

    public function getJoinDate(): ?\DateTimeInterface
    {
        return $this->joinDate;
    }

    public function setJoinDate(\DateTimeInterface $joinDate): static
    {
        $this->joinDate = $joinDate;
        return $this;
    }

    public function getFriendsSent(): Collection
    {
        return $this->friendsSent;
    }

    public function getFriendsReceived(): Collection
    {
        return $this->friendsReceived;
    }
}
