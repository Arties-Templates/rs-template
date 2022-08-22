if [[ -z $1 ]]; then
  echo "You must provid a tag to release"
  exit
fi

git-cliff --tag "$1" > CHANGELOG.md

git add CHANGELOG.md

git commit -m "chore(release): $1"
git tag "$1"

echo "Finished. Please push the commit"