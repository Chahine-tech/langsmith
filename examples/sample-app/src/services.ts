export async function fetchUser(id: string) {
  const response = await fetch(`/api/users/${id}`);
  if (!response.ok) {
    throw new Error("Failed to fetch user");
  }
  return response.json();
}

export function formatDate(date: Date): string {
  return date.toLocaleDateString("fr-FR");
}

const API_URLS = {
  users: "/api/users",
  posts: "/api/posts",
  comments: "/api/comments",
};
