-- Create AdventureNode table
CREATE TABLE IF NOT EXISTS AdventureNode (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    node_type VARCHAR(50) NOT NULL,
    body TEXT NOT NULL
);

-- Create Character table
CREATE TABLE IF NOT EXISTS Character (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    current_node_id UUID,
    FOREIGN KEY (current_node_id) REFERENCES AdventureNode(id)
);

-- Create AdventureSpot table
CREATE TABLE IF NOT EXISTS AdventureSpot (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL
);

-- Create AdventureOption table
CREATE TABLE IF NOT EXISTS AdventureOption (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    body TEXT NOT NULL,
    next_node_id UUID,
    FOREIGN KEY (next_node_id) REFERENCES AdventureNode(id)
);

-- Create AdventureSpotAdventureNode join table
CREATE TABLE IF NOT EXISTS AdventureSpotAdventureNode (
    adventure_spot_id UUID NOT NULL,
    adventure_node_id UUID NOT NULL,
    PRIMARY KEY (adventure_spot_id, adventure_node_id),
    FOREIGN KEY (adventure_spot_id) REFERENCES AdventureSpot(id),
    FOREIGN KEY (adventure_node_id) REFERENCES AdventureNode(id)
);

-- Create AdventureNodeAdventureOption join table
CREATE TABLE IF NOT EXISTS AdventureNodeAdventureOption (
    adventure_node_id UUID NOT NULL,
    adventure_option_id UUID NOT NULL,
    PRIMARY KEY (adventure_node_id, adventure_option_id),
    FOREIGN KEY (adventure_node_id) REFERENCES AdventureNode(id),
    FOREIGN KEY (adventure_option_id) REFERENCES AdventureOption(id)
);
